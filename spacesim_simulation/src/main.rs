use bevy::app::{App, PluginGroup, ScheduleRunnerPlugin};
use bevy::log::{info, LogPlugin};
use std::time::Duration;

use bevy::prelude::{EventReader, PostUpdate};
use bevy::MinimalPlugins;
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
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0))),
            LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_utility_ai=debug".into(),
                level: bevy::log::Level::DEBUG,
            },
            SimulationPlugin,
        ))
        .add_systems(PostUpdate, log_ai_updated_action)
        .run();
}
