pub mod common;
pub mod economy;
pub mod planet;
pub mod ships;
mod worldgen;

use bevy::prelude::{App, Plugin};
use bevy_utility_ai::systems::UtilityAIPlugin;
use economy::plugin::EconomySimulationPlugin;
use planet::plugin::AstralBodySimulationPlugin;
use ships::plugin::ShipSimulationPlugin;
use worldgen::create_world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EconomySimulationPlugin)
            .add_plugin(AstralBodySimulationPlugin)
            .add_plugin(ShipSimulationPlugin)
            .add_plugin(UtilityAIPlugin)
            .add_startup_system(create_world);
    }
}
