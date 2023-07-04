use bevy::app::FixedUpdate;
use bevy::log::{info, warn};
use bevy::prelude::{App, Commands, Entity, Query, Res, Time, Vec3, With, Without};
use bevy_utility_ai::ActionTarget;
use strum::IntoEnumIterator;

use crate::economy::commodity_type::CommodityType;
use crate::economy::components::{CommodityStorage, Wealth};
use crate::economy::market::Market;
use crate::economy::market_wq::{
    MarketSellerMutQuery, MarketSellerMutQueryItem, MarketSellerQuery,
};
use crate::economy::system_market_info::{MarketTradePotential, SystemMarketInfo};
use crate::ships::ai::ActionPurchaseGoodsFromMarket;
use crate::{common::marker_components::IsPlanet, planet::components::OnPlanet};

use super::{ai::ActionMoveToPlanet, components::SystemCoordinates};

const TRAVEL_TO_PLANET_STEPSIZE: f32 = 10_000_000.0;

pub(crate) fn travel_to_planet(
    time: Res<Time>,
    mut commands: Commands,
    mut q_subject: Query<
        (Entity, &mut SystemCoordinates, &ActionTarget),
        (With<ActionMoveToPlanet>, Without<IsPlanet>),
    >,
    q_target: Query<(Entity, &SystemCoordinates), With<IsPlanet>>,
) {
    let step_size = time.delta_seconds() * TRAVEL_TO_PLANET_STEPSIZE;
    for (subject, mut subject_coords, target_entity) in q_subject.iter_mut() {
        if let Ok((target, target_coords)) = q_target.get(target_entity.target) {
            let travel_vector: Vec3 = target_coords.value - subject_coords.value;
            if travel_vector.length() >= step_size {
                // move to planet
                subject_coords.value += travel_vector.normalize() * step_size;
            } else {
                // dock on planet
                commands.entity(subject).insert(OnPlanet { value: target });
                commands.entity(subject).remove::<SystemCoordinates>();
                info!(
                    "entity {:?} docked on planet {:?}",
                    subject, target_entity.target
                );
            }
        }
    }
}

pub(crate) fn purchase_goods_from_market(
    mut q_subject: Query<
        (Entity, &OnPlanet, &mut CommodityStorage, &mut Wealth),
        With<ActionPurchaseGoodsFromMarket>,
    >,
    mut q_market: Query<&mut Market>,
    q_market_seller: Query<MarketSellerQuery>,
    mut q_market_seller_mut: Query<MarketSellerMutQuery>,
    r_time: Res<Time>,
) {
    for (subject_entity_id, on_planet, mut commodity_storage, mut wealth) in
        q_subject.iter_mut()
    {
        if let Some(mut market) = q_market.get_mut(on_planet.value).ok() {
            // TODO: buying the most discounted goods isn't the most robust method here
            let mut commodity_demand: Vec<(CommodityType, f32)> = CommodityType::iter()
                .zip(market.demand_price_modifier)
                .collect();
            commodity_demand.sort_by(|a, b| a.1.total_cmp(&b.1));
            for commodity in commodity_demand.iter().map(|(com, _)| com) {
                let mut purchase_quotes = market.get_buyer_quotes(
                    subject_entity_id,
                    *commodity,
                    commodity_storage.available_capacity,
                    &q_market_seller,
                );
                while commodity_storage.available_capacity > 0.01 && wealth.value > 0.01 {
                    if let Some(mut quote) = purchase_quotes.pop_front() {
                        let desired_units =
                            f32::min(quote.units, commodity_storage.available_capacity);
                        quote.units = desired_units;
                        let MarketSellerMutQueryItem {
                            storage: mut seller_commodity_storage,
                            wealth: mut seller_wealth,
                            ..
                        } = q_market_seller_mut.get_mut(quote.seller).unwrap();
                        let purchase = market.buy(
                            &quote,
                            &mut commodity_storage,
                            &mut wealth,
                            &mut seller_commodity_storage,
                            &mut seller_wealth,
                            &r_time,
                        );
                        match purchase {
                            Ok(_) => {
                                info!("succesfully bought goods from market: {:?}", quote)
                            }
                            Err(err) => info!("unable to purchase from market: {}", err),
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn register_actions(app: &mut App) {
    app.add_systems(FixedUpdate, (travel_to_planet, purchase_goods_from_market));
}
