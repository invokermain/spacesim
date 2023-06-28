use bevy::app::{CoreSchedule, IntoSystemAppConfig};
use bevy::log::info;
use bevy::prelude::{App, Commands, Entity, Query, Res, Time, Vec3, With, Without};
use bevy_utility_ai::ActionTarget;

use crate::economy::market::Market;
use crate::{common::marker_components::IsPlanet, planet::components::OnPlanet};

use super::{ai::ActionMoveToPlanet, components::SystemCoordinates};

const TRAVEL_TO_PLANET_STEPSIZE: f32 = 100_000_000.0;

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
    q_subject: Query<(Entity, &ActionTarget), With<ActionMoveToPlanet>>,
    q_market: Query<&mut Market>,
) {
}

pub(crate) fn register_actions(app: &mut App) {
    app.add_systems((
        travel_to_planet.in_schedule(CoreSchedule::FixedUpdate),
        purchase_goods_from_market.in_schedule(CoreSchedule::FixedUpdate),
    ));
}
