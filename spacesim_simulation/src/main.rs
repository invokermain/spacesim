use bevy::app::{App, PluginGroup, ScheduleRunnerPlugin, Update};
use bevy::log::{info, LogPlugin};
use std::time::Duration;

use bevy::prelude::{EventReader, IntoSystemConfigs};
use bevy::DefaultPlugins;
use bevy_utility_ai::plugin::UtililityAISet;
use bevy_utility_ai::systems::make_decisions::EntityActionChangeEvent;
use spacesim_simulation::SimulationPlugin;

fn log_ai_updated_action(mut e_update_action: EventReader<EntityActionChangeEvent>) {
    for event in e_update_action.iter() {
        info!(
            "entity {:?} | decided {} | target {:?} | score {:.2}",
            event.entity_id, event.new_action, event.new_target, event.new_score,
        )
    }
}

fn main() {
    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0)),
            DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu=error,naga=warn,bevy_utility_ai=info".into(),
                level: bevy::log::Level::DEBUG,
            }),
            SimulationPlugin,
        ))
        .add_systems(
            Update,
            log_ai_updated_action.in_set(UtililityAISet::UpdateActions),
        )
        .run();
}
