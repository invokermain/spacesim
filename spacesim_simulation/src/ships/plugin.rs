use bevy::prelude::{App, Plugin};

use super::ai::define_ship_ai;

pub struct ShipSimulationPlugin;

impl Plugin for ShipSimulationPlugin {
    fn build(&self, app: &mut App) {
        define_ship_ai(app);
    }
}
