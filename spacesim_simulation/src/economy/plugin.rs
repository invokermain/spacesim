use super::systems::{company_simulate, population_consumption, update_market_statistics};
use crate::economy::system_market_info::{update_system_market_info, SystemMarketInfo};
use bevy::app::FixedUpdate;
use bevy::prelude::{IntoSystemConfigs, IntoSystemSetConfig, Plugin, SystemSet};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum EconomySimulationSet {
    Simulate,
    Aggregate,
}

pub struct EconomySimulationPlugin;

impl Plugin for EconomySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SystemMarketInfo>()
            .add_systems(
                FixedUpdate,
                (company_simulate, population_consumption)
                    .in_set(EconomySimulationSet::Simulate),
            )
            .add_systems(
                FixedUpdate,
                (update_market_statistics, update_system_market_info)
                    .in_set(EconomySimulationSet::Aggregate),
            )
            .configure_set(
                FixedUpdate,
                EconomySimulationSet::Aggregate.after(EconomySimulationSet::Simulate),
            );
    }
}
