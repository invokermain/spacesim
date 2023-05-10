use bevy::prelude::Bundle;

use super::components::SystemCoordinates;

#[derive(Bundle)]
pub struct ShipBundle {
    pub system_coordinates: SystemCoordinates,
}
