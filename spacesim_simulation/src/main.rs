use bevy::app::{App, ScheduleRunnerSettings};
use bevy::log::{info, LogPlugin};
use std::time::Duration;

use bevy::prelude::EventReader;
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
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            // 144 fps
            1.0 / 144.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_utility_ai=info".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugin(SimulationPlugin)
        .add_system(log_ai_updated_action)
        .run();
}
