use bevy::prelude::Entity;

use crate::economy::components::CommodityArr;

#[derive(Debug)]
pub struct RenderCompany {
    pub entity: Entity,
    pub wealth: f32,
    pub commodity_storage: CommodityArr<f32>,
}
