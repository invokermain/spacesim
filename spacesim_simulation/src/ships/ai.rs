use bevy::{
    prelude::{Component, Entity, Query, ReflectComponent, ReflectDefault, With},
    reflect::Reflect,
    utils::HashMap,
};

use crate::common::marker_components::IsPlanet;

use super::components::SystemCoordinates;

// Marker component for our AI system
#[derive(Component)]
pub struct ShipAI {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
struct ActionTravellingTo {}

pub(crate) fn distance_to_planet(
    ship_coords: &SystemCoordinates,
    planets: Query<(Entity, &SystemCoordinates), With<IsPlanet>>,
) -> HashMap<Entity, f32> {
    HashMap::from_iter(planets.iter().map(|(planet_entity, planet_coords)| {
        (
            planet_entity,
            ship_coords.value.distance(planet_coords.value),
        )
    }))
}
