use bevy::prelude::{App, Commands, Entity, Query, Vec3, With, Without};
use bevy_utility_ai::ActionTarget;

use crate::{common::marker_components::IsPlanet, planet::components::OnPlanet};

use super::{ai::ActionMoveToPlanet, components::SystemCoordinates};

// TODO: what if the target is also a ship that's travelling somewhere?
pub(crate) fn travel_to_planet(
    mut commands: Commands,
    mut q_subject: Query<
        (Entity, &mut SystemCoordinates, &ActionTarget),
        (With<ActionMoveToPlanet>, Without<IsPlanet>),
    >,
    q_target: Query<(Entity, &SystemCoordinates), With<IsPlanet>>,
) {
    let travel_to_planet_stepsize = 25_000.0;
    for (subject, mut subject_coors, target_entity) in q_subject.iter_mut() {
        if let Ok((target, target_coords)) = q_target.get(target_entity.target) {
            let travel_vector: Vec3 = target_coords.value - subject_coors.value;
            if travel_vector.length() < travel_to_planet_stepsize {
                // dock on planet
                commands.entity(subject).insert(OnPlanet { value: target });
            } else {
                // move to planet
                subject_coors.value += travel_vector.normalize() * travel_to_planet_stepsize;
            }
        }
    }
}

pub(crate) fn register_actions(app: &mut App) {
    app.add_systems((travel_to_planet,));
}
