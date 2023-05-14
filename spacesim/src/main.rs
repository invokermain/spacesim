mod game;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::{App, PluginGroup},
    utils::default,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use game::GamePlugin;
use spacesim_simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin {
            filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ..default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(GamePlugin)
        .run();
}
