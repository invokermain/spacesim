pub mod common;
pub mod economy;
pub mod ships;
mod worldgen;

use bevy::prelude::{App, Plugin};

use economy::plugin::EconomySimulationPlugin;

use worldgen::create_world;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EconomySimulationPlugin)
            .add_startup_system(create_world);
    }
}
