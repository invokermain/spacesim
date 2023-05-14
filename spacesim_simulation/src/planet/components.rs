use bevy::prelude::{Bundle, Component, Entity};

use crate::common::components::Name;
use crate::common::marker_components::IsPlanet;
use crate::economy::commodity_type::CommodityArr;
use crate::economy::components::Wealth;
use crate::economy::market::Market;
use crate::ships::components::SystemCoordinates;

/// Any entity that exists on a given Planet should have this.
#[derive(Component, Clone, Copy)]
pub struct OnPlanet {
    pub value: Entity,
}

/// Describes the current population of the Planet.
#[derive(Component, Clone, Copy)]
pub struct Population {
    pub consumption: CommodityArr<f32>,
}

/// A list of all the companies that exist on the planet.
#[derive(Component, Clone)]
pub struct Companies {
    pub value: Vec<Entity>,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub name: Name,
    pub market: Market,
    pub population: Population,
    pub companies: Companies,
    pub wealth: Wealth,
    pub coordinates: SystemCoordinates,
    pub marker: IsPlanet,
}

impl PlanetBundle {
    pub fn new(name: String, population: Population, coordinates: SystemCoordinates) -> Self {
        Self {
            name: Name { value: name },
            market: Market::default(),
            population,
            companies: Companies { value: Vec::new() },
            wealth: Wealth {
                value: f32::INFINITY,
            },
            coordinates,
            marker: IsPlanet {},
        }
    }
}
