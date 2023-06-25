use bevy::prelude::App;
use bevy::{
    prelude::{Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::considerations::Consideration;

use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{LinearCurve, PolynomialCurve};
use bevy_utility_ai::{input_system, targeted_input_system};

use super::components::SystemCoordinates;
use crate::common::marker_components::IsPlanet;
use crate::economy::components::CommodityStorage;
use crate::economy::market::Market;

// Marker component for our AI system
#[derive(Component)]
pub struct ShipAI {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionMoveToPlanet {}

/// Distance between two coordinates in km. Range: 0 -> f32::MAX
#[targeted_input_system]
pub(crate) fn system_distance(
    subject: (&SystemCoordinates,),
    target: (&SystemCoordinates,),
) -> f32 {
    subject.0.value.distance(target.0.value)
}

/// Ratio of available storage capacity. 1 implies hold is empty. Range: 0 -> 1
#[input_system]
pub(crate) fn free_hold_space_ratio(storage: &CommodityStorage) -> f32 {
    storage.available_capacity / storage.max_capacity
}

#[input_system]
pub(crate) fn in_space(storage: &CommodityStorage) -> f32 {
    storage.available_capacity / storage.max_capacity
}

pub(super) fn define_ship_ai(app: &mut App) {
    DefineAI::<ShipAI>::new()
        .add_decision::<ActionMoveToPlanet>(vec![
            Consideration::targeted_filter::<IsPlanet>(),
            Consideration::targeted_filter::<Market>(),
            // Consider the planets near me
            Consideration::targeted(system_distance)
                .with_response_curve(LinearCurve::new(-1.0 / 150_000_000.0).shifted(0.0, 1.0))
                .set_input_name("Distance to nearby planets"),
            // Considers how much available hold space I have
            Consideration::simple(free_hold_space_ratio)
                .with_response_curve(PolynomialCurve::new(1.0, 3.0))
                .set_input_name("Available hold capacity"),
        ])
        .register(app);
}

// Actions
// - TravelToPlanet(Target)

// Decision "go to planet so we can buy stuff", action "TravelToPlanet(Target)"
// - am I in space
// - how empty my hold is
// - for each planet in system:
//    - distance to the planet
//    - how discounted goods are on the planet

// Decision "go to planet so we can sell stuff", action "TravelToPlanet(Target)"
// - am I in space
// - how full my hold is
// - for each planet in system:
//    - distance to the planet
//    - potential profit from sales on the planet

// Decision "purchase goods from market", action "PurchaseCommodities"
// - am I on a planet
// - how empty my hold is
// - how discounted goods are on the planet I am on

// Decision "sell goods to market", action "SellCommodities"
// - am I on a planet
// - how full my hold is
// - potential profit from sales on the planet I am on
