use super::systems::{company_simulate, population_consumption, update_market_statistics};
use crate::economy::system_market_info::{update_system_market_info, SystemMarketInfo};
use bevy::app::{CoreSchedule, IntoSystemAppConfig};
use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig, Plugin, SystemSet};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum EconomySimulationSet {
    Simulate,
    Aggregate,
}

pub struct EconomySimulationPlugin;

impl Plugin for EconomySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SystemMarketInfo>()
            .add_system(
                company_simulate
                    .in_set(EconomySimulationSet::Simulate)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(
                population_consumption
                    .in_set(EconomySimulationSet::Simulate)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(
                update_market_statistics
                    .in_set(EconomySimulationSet::Aggregate)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(
                update_system_market_info
                    .in_set(EconomySimulationSet::Aggregate)
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .configure_set(
                EconomySimulationSet::Aggregate.after(EconomySimulationSet::Simulate),
            );
    }
}
