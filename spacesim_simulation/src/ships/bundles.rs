use crate::common::components::Name;
use bevy::prelude::Bundle;

use crate::common::marker_components::IsShip;
use crate::economy::components::CommodityStorage;
use crate::ships::ai::ShipAI;

use super::components::SystemCoordinates;

#[derive(Bundle)]
pub struct ShipBundle {
    name: Name,
    system_coordinates: SystemCoordinates,
    commodity_storage: CommodityStorage,
    ai: ShipAI,
    marker: IsShip,
}

impl ShipBundle {
    pub fn new(name: Name, system_coordinates: SystemCoordinates) -> Self {
        Self {
            name,
            system_coordinates,
            commodity_storage: CommodityStorage::new(5.0),
            marker: IsShip {},
            ai: ShipAI {},
        }
    }
}
