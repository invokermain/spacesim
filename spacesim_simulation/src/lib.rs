pub mod common;
pub mod economy;
pub mod planet;
pub mod ships;
mod worldgen;

use bevy::prelude::{App, FixedTime, Plugin};
use bevy_utility_ai::plugin::UtilityAIPlugin;
use economy::plugin::EconomySimulationPlugin;
use planet::plugin::AstralBodySimulationPlugin;
use ships::plugin::ShipSimulationPlugin;
use worldgen::create_world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app // Library Plugins
            .add_plugin(UtilityAIPlugin)
            // Setup
            .insert_resource(FixedTime::new_from_secs(1.0 / 4.0))
            // Game Plugins
            .add_plugin(EconomySimulationPlugin)
            .add_plugin(AstralBodySimulationPlugin)
            .add_plugin(ShipSimulationPlugin)
            .add_startup_system(create_world);
    }
}
