pub mod common;
pub mod economy;
pub mod planet;
pub mod ships;
mod worldgen;

use bevy::app::{FixedUpdate, Startup};
use bevy::prelude::{App, FixedTime, Plugin};
use bevy_utility_ai::plugin::UtilityAIPlugin;
use economy::plugin::EconomySimulationPlugin;
use planet::plugin::AstralBodySimulationPlugin;
use ships::plugin::ShipSimulationPlugin;
use worldgen::create_world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup
            .insert_resource(FixedTime::new_from_secs(1.0 / 4.0))
            // Library Plugins
            .add_plugins(UtilityAIPlugin::new(FixedUpdate))
            // Game Plugins
            .add_plugins((
                EconomySimulationPlugin,
                AstralBodySimulationPlugin,
                ShipSimulationPlugin,
            ))
            .add_systems(Startup, create_world);
    }
}
