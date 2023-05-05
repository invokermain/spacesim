use bevy::prelude::Entity;

use crate::economy::{commodity_type::CommodityArr, components::Population, market::Market};

#[derive(Debug)]
pub struct RenderCompany {
    pub entity: Entity,
    pub wealth: f32,
    pub commodity_storage: CommodityArr<f32>,
}

pub struct RenderSystemInfo {
    pub planets: Vec<Entity>,
}

pub struct RenderPlanet {
    pub entity: Entity,
    pub population: Population,
    pub market: Market,
}
