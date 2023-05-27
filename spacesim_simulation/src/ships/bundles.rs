use bevy::prelude::Bundle;

use crate::common::marker_components::IsShip;

use super::components::SystemCoordinates;

#[derive(Bundle)]
pub struct ShipBundle {
    system_coordinates: SystemCoordinates,
    marker: IsShip,
}

impl ShipBundle {
    pub fn new(system_coordinates: SystemCoordinates) -> Self {
        Self {
            system_coordinates,
            marker: IsShip {},
        }
    }
}
