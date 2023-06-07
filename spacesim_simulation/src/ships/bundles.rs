use bevy::prelude::Bundle;

use crate::common::marker_components::IsShip;
use crate::ships::ai::ShipAI;

use super::components::SystemCoordinates;

#[derive(Bundle)]
pub struct ShipBundle {
    system_coordinates: SystemCoordinates,
    ai: ShipAI,
    marker: IsShip,
}

impl ShipBundle {
    pub fn new(system_coordinates: SystemCoordinates) -> Self {
        Self {
            system_coordinates,
            marker: IsShip {},
            ai: ShipAI {},
        }
    }
}
