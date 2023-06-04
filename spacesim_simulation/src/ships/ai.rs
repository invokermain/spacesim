use bevy::ecs::query::WorldQuery;
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

#[derive(WorldQuery)]
struct SystemPlanets {}

// #[targeted_input(SystemPlanets)]
pub(crate) fn distance_to_planet(subject: &SystemCoordinates, target: &SystemCoordinates) -> f32 {
    subject.value.distance(target.value)
}

// Actions
// - TravelToPlanet(Target)

// Decision "go to planet so we can buy stuff", action "TravelToPlanet(Target)"
// - am I in space
// - how empty my hold is
// for each planet in system:
//  - distance to the planet
//  - how discounted goods are on the planet

// ->
// for each AIMeta

// Decision "go to planet so we can sell stuff", action "TravelToPlanet(Target)"
// - am I in space
// - how full my hold is
// - distance to the planet
// - potential profit from sales on the planet

// Decision "purchase goods from market", action "PurchaseCommodities"
// - am I on a planet
// - how empty my hold is
// - how discounted goods are on the planet

// Decision "sell goods to market", action "SellCommodities"
// - am I on a planet
// - how full my hold is
// - potential profit from sales on the planet
