use std::time::Duration;

use bevy::{
    prelude::{IntoSystemConfig, IntoSystemSetConfig, Plugin, SystemSet},
    time::common_conditions::on_timer,
};

use super::systems::{company_simulate, population_consumption, update_market_statistics};

const SIMULATION_TICK_RATE: f32 = 0.25;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum EconomySimulationSet {
    Simulate,
    Aggregate,
}

pub struct EconomySimulationPlugin;

impl Plugin for EconomySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(
            company_simulate
                .run_if(on_timer(Duration::from_secs_f32(SIMULATION_TICK_RATE)))
                .in_set(EconomySimulationSet::Simulate),
        )
        .add_system(
            population_consumption
                .run_if(on_timer(Duration::from_secs_f32(SIMULATION_TICK_RATE)))
                .in_set(EconomySimulationSet::Simulate),
        )
        .add_system(
            update_market_statistics
                .run_if(on_timer(Duration::from_secs_f32(SIMULATION_TICK_RATE)))
                .in_set(EconomySimulationSet::Aggregate),
        )
        .configure_set(EconomySimulationSet::Aggregate.after(EconomySimulationSet::Simulate));
    }
}
