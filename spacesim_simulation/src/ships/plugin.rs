use bevy::prelude::{App, Plugin};

use super::{actions::register_actions, ai::define_ship_ai};

pub struct ShipSimulationPlugin;

impl Plugin for ShipSimulationPlugin {
    fn build(&self, app: &mut App) {
        define_ship_ai(app);
        register_actions(app);
    }
}
