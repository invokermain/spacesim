pub mod common;
pub mod economy;
pub mod planet;
pub mod ships;
mod worldgen;

use bevy::prelude::{App, Plugin};
use economy::plugin::EconomySimulationPlugin;
use planet::plugin::AstralBodySimulationPlugin;
use worldgen::create_world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EconomySimulationPlugin)
            .add_plugin(AstralBodySimulationPlugin)
            .add_startup_system(create_world);
    }
}