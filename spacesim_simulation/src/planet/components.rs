use bevy::prelude::{Bundle, Component, Entity};
use std::borrow::Cow;

use crate::common::components::Name;
use crate::common::marker_components::IsPlanet;
use crate::economy::components::CommodityConsumer;
use crate::economy::market::Market;
use crate::ships::components::SystemCoordinates;

/// Any entity that exists on a given Planet should have this.
#[derive(Component, Clone, Copy)]
pub struct OnPlanet {
    pub value: Entity,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub name: Name,
    pub market: Market,
    pub commodity_consumer: CommodityConsumer,
    pub coordinates: SystemCoordinates,
    pub marker: IsPlanet,
}

impl PlanetBundle {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        coordinates: SystemCoordinates,
        commodity_consumer: CommodityConsumer,
    ) -> Self {
        Self {
            name: Name::new(name),
            market: Market::default(),
            commodity_consumer,
            coordinates,
            marker: IsPlanet {},
        }
    }
}
