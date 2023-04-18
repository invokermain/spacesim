use bevy::prelude::Entity;

use crate::economy::{
    components::{CommodityArr, Population},
    market::Market,
};

#[derive(Debug)]
pub struct RenderCompany {
    pub entity: Entity,
    pub wealth: f32,
    pub commodity_storage: CommodityArr<f32>,
}

pub struct RenderPlanet<'a> {
    pub entity: Entity,
    pub population: &'a Population,
    pub market: &'a Market,
}
