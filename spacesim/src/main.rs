#[cfg(debug_assertions)]
mod debugger;
mod game;

use crate::debugger::DebuggerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::{
    prelude::{App, PluginGroup},
    utils::default,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_vector_shapes::painter::ShapeConfig;
use bevy_vector_shapes::prelude::Alignment;
use bevy_vector_shapes::ShapePlugin;
use game::GamePlugin;
use spacesim_simulation::SimulationPlugin;

fn main() {
    let mut app = App::new();

    if cfg!(debug_assertions) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_utility_ai=info".into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DebuggerPlugin);
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "warn,wgpu_core=warn,wgpu_hal=warn".into(),
                    level: bevy::log::Level::WARN,
                }),
        );
    }

    app.add_plugin(SimulationPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ShapePlugin {
            base_config: ShapeConfig {
                alignment: Alignment::Billboard,
                ..ShapeConfig::default_3d()
            },
            ..default()
        });

    app.run();
}
