use bevy::prelude::Bundle;

use crate::common::marker_components::IsShip;

use super::components::SystemCoordinates;

#[derive(Bundle)]
pub struct ShipBundle {
    pub system_coordinates: SystemCoordinates,
    pub marker: IsShip,
}

impl ShipBundle {
    pub fn new(coordinates: SystemCoordinates) -> Self {
        Self {
            system_coordinates: coordinates,
            marker: IsShip {},
        }
    }
}
