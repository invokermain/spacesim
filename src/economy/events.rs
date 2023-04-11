use bevy::prelude::Entity;

use super::components::CommodityType;

pub struct CommodityProducedEvent {
    pub source_entity: Entity,
    pub commodity_type: CommodityType,
    pub change: f32,
}
