use std::time::Duration;

use bevy::{
    prelude::{IntoSystemConfig, Plugin},
    time::common_conditions::on_timer,
};

use crate::common::SIMULATION_TICK_RATE;

use super::systems::orbit_planetary_body;

pub struct AstralBodySimulationPlugin;

impl Plugin for AstralBodySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(
            orbit_planetary_body.run_if(on_timer(Duration::from_secs_f32(SIMULATION_TICK_RATE))),
        );
    }
}
