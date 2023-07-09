use bevy::app::FixedUpdate;
use bevy::log::info;
use bevy::prelude::{App, Commands, Entity, Query, Res, Time, Vec3, With, Without};
use bevy_utility_ai::ActionTarget;
use strum::IntoEnumIterator;

use crate::economy::commodity_type::CommodityType;
use crate::economy::components::{CommodityStorage, Wealth};
use crate::economy::market::Market;
use crate::ships::ai::{ActionPurchaseGoodsFromMarket, ActionTakeOffFromPlanet};
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
            let (commodity_type, _) = commodity_demand.first().unwrap();
            let commodity_idx = *commodity_type as usize;

            let purchasable_quantity = f32::min(
                f32::min(
                    market.total_supply[commodity_idx],
                    commodity_storage.available_capacity,
                ),
                // Note: we buy in granular steps so that our AI has a chance to decide to
                //       do something else when they have bought some goods, e.g. refuel if
                //       money is low.
                commodity_storage.max_capacity * 0.20,
            );

            let unit_price = market.unit_price(*commodity_type);

            // only purchase what we can afford
            let units = f32::min(purchasable_quantity, (wealth.value * unit_price) - 0.05);

            match market.purchase_commodity(
                subject_entity_id,
                &mut wealth,
                &mut commodity_storage,
                *commodity_type,
                units,
                r_time.elapsed_seconds()
            ) {
                Ok(transaction) => info!(
                    "entity {:?} | purchased {:.1} units of {:?} | price {:.2} | wealth {:.2} -> {:.2}",
                    transaction.transaction_entity,
                    transaction.units,
                    transaction.commodity_type,
                    transaction.transaction_cost,
                    wealth.value + transaction.transaction_cost,
                    wealth.value,
                ),
                Err(err) => info!("unable to purchase goods from market: {:?}", err),
            }
        }
    }
}

pub(crate) fn take_off_from_planet(
    q_subject: Query<(Entity, &OnPlanet), With<ActionTakeOffFromPlanet>>,
    q_planet: Query<&SystemCoordinates, With<IsPlanet>>,
    mut commands: Commands,
) {
    for (subject_entity_id, on_planet) in q_subject.iter() {
        let planet_coordinates = q_planet.get(on_planet.value).unwrap();
        commands
            .entity(subject_entity_id)
            .remove::<OnPlanet>()
            .insert(planet_coordinates.clone());
    }
}

pub(crate) fn register_actions(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            travel_to_planet,
            purchase_goods_from_market,
            take_off_from_planet,
        ),
    );
}
