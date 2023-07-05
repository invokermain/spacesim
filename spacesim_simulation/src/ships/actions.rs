use bevy::app::FixedUpdate;
use bevy::log::info;
use bevy::prelude::{App, Commands, Entity, Query, Res, Time, Vec3, With, Without};
use bevy_utility_ai::ActionTarget;
use strum::IntoEnumIterator;

use crate::economy::commodity_type::CommodityType;
use crate::economy::components::{CommodityStorage, Wealth};
use crate::economy::market::{Market, Transaction};
use crate::economy::market_wq::{MarketSellerMutQuery, MarketSellerMutQueryReadOnlyItem};
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
    mut q_market_seller_mut: Query<
        MarketSellerMutQuery,
        Without<ActionPurchaseGoodsFromMarket>,
    >,
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

            // Purchase the cheapest commodity
            if let Some((commodity_type, _)) = commodity_demand.first() {
                let commodity_idx = *commodity_type as usize;
                let mut cheapest_seller: Option<MarketSellerMutQueryReadOnlyItem> = None;
                for market_member in &market.market_members {
                    let next_seller = q_market_seller_mut.get(*market_member).ok();
                    if let Some(next_seller) = next_seller {
                        if next_seller.storage.storage[commodity_idx] <= 0.1 {
                            continue;
                        }
                        match &cheapest_seller {
                            None => cheapest_seller = Some(next_seller),
                            Some(current_seller) => {
                                if next_seller.pricing.value[commodity_idx]
                                    < current_seller.pricing.value[commodity_idx]
                                {
                                    cheapest_seller = Some(next_seller);
                                }
                            }
                        }
                    }
                }

                if cheapest_seller.is_none() {
                    continue;
                }

                // Purchase from the cheapest seller
                if let Some(mut seller) = q_market_seller_mut
                    .get_mut(cheapest_seller.unwrap().entity)
                    .ok()
                {
                    let seller_commodity_quantity = seller.storage.storage[commodity_idx];
                    let purchase_quantity = f32::min(
                        f32::min(
                            seller_commodity_quantity,
                            commodity_storage.available_capacity,
                        ),
                        // Note: we buy in granular steps so that our AI has a chance to decide to
                        //       do something else when thye have bough some goods, e.g. refuel if
                        //       money is low.
                        commodity_storage.max_capacity * 0.20,
                    );

                    let unit_price = market.demand_price_modifier[commodity_idx]
                        * seller.pricing.value[commodity_idx];

                    // only purchase what we can afford
                    let units =
                        f32::min(purchase_quantity, (wealth.value * unit_price) - 0.05);

                    let transaction = Transaction {
                        buyer: subject_entity_id,
                        seller: seller.entity,
                        commodity_type: *commodity_type,
                        units,
                        unit_price,
                    };

                    match market.buy(
                        &transaction,
                        &mut commodity_storage,
                        &mut wealth,
                        seller.storage.as_mut(),
                        seller.wealth.as_mut(),
                        r_time.elapsed_seconds(),
                    ) {
                        Ok(_) => info!(
                            "entity {:?} | purchased {:.1} units of {:?} | price {:.2} | wealth {:.2} -> {:.2}",
                            transaction.buyer,
                            transaction.units,
                            transaction.commodity_type,
                            transaction.units * transaction.unit_price,
                            wealth.value + transaction.units * transaction.unit_price,
                            wealth.value,
                        ),
                        Err(err) => info!("unable to purchase goods from market: {:?}", err),
                    }
                }
            }
        }
    }
}

pub(crate) fn register_actions(app: &mut App) {
    app.add_systems(FixedUpdate, (travel_to_planet, purchase_goods_from_market));
}
