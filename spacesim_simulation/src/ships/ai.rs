use bevy::prelude::{App, Entity, Res};
use bevy::{
    prelude::{Component, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::considerations::Consideration;

use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{LinearCurve, PolynomialCurve};
use bevy_utility_ai::{input_system, targeted_input_system};

use super::components::SystemCoordinates;
use crate::common::marker_components::IsPlanet;
use crate::economy::components::{CommodityStorage, Wealth};
use crate::economy::market::Market;
use crate::economy::system_market_info::SystemMarketInfo;
use crate::planet::components::OnPlanet;

// Marker component for our AI system
#[derive(Component)]
pub struct ShipAI {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionMoveToPlanet {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionPurchaseGoodsFromMarket {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionTakeOffFromPlanet {}

/// Distance between two coordinates in km. Range: 0 -> f32::MAX
#[targeted_input_system]
pub(crate) fn distance_to_nearby_planet(
    subject: (&SystemCoordinates,),
    target: (&SystemCoordinates,),
) -> f32 {
    subject.0.value.distance(target.0.value)
}

/// Ratio of available storage capacity. 1 implies hold is empty. Range: 0 -> 1
#[input_system]
pub(crate) fn available_hold_capacity(storage: &CommodityStorage) -> f32 {
    storage.available_capacity / storage.max_capacity
}

/// The subject's available wealth. Range: 0 -> ~1000
#[input_system]
pub(crate) fn available_wealth(wealth: &Wealth) -> f32 {
    wealth.value
}

/// Total trade potential of a Market. Range: 0 -> ~1000
#[targeted_input_system]
pub(crate) fn market_trade_potential(
    target: (Entity,),
    res_system_market_info: Res<SystemMarketInfo>,
) -> f32 {
    *res_system_market_info
        .market_total_trade_potential
        .get(&target.0)
        .unwrap_or(&0.0)
}

pub(super) fn define_ship_ai(app: &mut App) {
    DefineAI::<ShipAI>::new()
        .add_decision(
            Decision::targeted::<ActionMoveToPlanet>()
                .target_filter_include::<IsPlanet>()
                .target_filter_include::<Market>()
                .subject_filter_exclude::<OnPlanet>()
                .add_consideration(
                    Consideration::targeted(distance_to_nearby_planet).with_response_curve(
                        LinearCurve::new(-1.0 / 150_000_000.0).shifted(0.0, 1.0),
                    ),
                )
                .add_consideration(
                    Consideration::simple(available_hold_capacity)
                        .with_response_curve(PolynomialCurve::new(1.0, 3.0)),
                )
                .add_consideration(
                    Consideration::targeted(market_trade_potential).with_response_curve(
                        PolynomialCurve::new(1.0, 0.1).shifted(-1.0, -1.0),
                    ),
                ),
        )
        .add_decision(
            Decision::simple::<ActionPurchaseGoodsFromMarket>()
                .subject_filter_include::<OnPlanet>()
                .add_consideration(
                    Consideration::simple(available_hold_capacity)
                        .with_response_curve(PolynomialCurve::new(1.0, 3.0)),
                )
                .add_consideration(
                    Consideration::simple(available_wealth).with_response_curve(
                        PolynomialCurve::new(1.0, 0.15).shifted(-1.0, -1.0),
                    ),
                ),
        )
        .add_decision(Decision::simple::<ActionTakeOffFromPlanet>().set_base_score(0.20))
        .register(app);
}

// Actions
// - TravelToPlanet(Target)

// Decision "go to planet so we can buy stuff", action "TravelToPlanet(Target)" ✔️
// - am I in space ✔️
// - how empty my hold is ✔️
// - for each planet in system:
//    - distance to the planet ✔️
//    - trade potential of the planet ✔️

// Decision "go to planet so we can sell stuff", action "TravelToPlanet(Target)"
// - am I in space
// - how full my hold is
// - for each planet in system:
//    - distance to the planet
//    - potential profit from sales on the planet

// Decision "purchase goods from market", action "PurchaseCommodities"
// Note: this might be the first use of a targeted consideration for a simple decision
// - am I on a planet ✔️
// - how empty my hold is ✔️
// - trade potential of commodities on planet (needs to factor in distance to other markets?) ❌
// - how much money I have ✔️

// Decision "take off from planet", action "TakeOffFromPlanet"
// - static low score value

// Decision "sell goods to market", action "SellCommodities"
// - am I on a planet
// - how full my hold is
// - potential profit from sales on the planet I am on
