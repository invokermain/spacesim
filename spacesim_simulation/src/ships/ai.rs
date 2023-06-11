use bevy::prelude::{App, IntoSystemConfig};
use bevy::{
    prelude::{Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::considerations::Consideration;

use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::plugin::UtililityAISet;
use bevy_utility_ai::response_curves::LinearCurve;
use bevy_utility_ai::targeted_input_system;

use crate::common::marker_components::IsPlanet;
use crate::planet::components::OnPlanet;

use super::actions::travel_to_planet;
use super::components::SystemCoordinates;

// Marker component for our AI system
#[derive(Component)]
pub struct ShipAI {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionMoveToPlanet {}

#[targeted_input_system]
pub(crate) fn distance_to_planet(
    subject: (&SystemCoordinates,),
    target: (&SystemCoordinates, &IsPlanet),
) -> f32 {
    subject.0.value.distance(target.0.value)
}

pub(super) fn define_ship_ai(app: &mut App) {
    DefineAI::<ShipAI>::new()
        .add_decision::<ActionMoveToPlanet>(vec![Consideration::targeted(distance_to_planet)
            .with_response_curve(LinearCurve::new(-1.0 / 75_000_000.0).shifted(0.0, 1.0))
            .set_input_name("distance_to_planet".into())])
        .register(app);

    app.register_type::<ActionMoveToPlanet>();
    app.add_system(distance_to_planet.in_set(UtililityAISet::CalculateInputs));
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
