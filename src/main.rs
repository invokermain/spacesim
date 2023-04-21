mod common;
mod economy;
mod ui;
mod worldgen;

use bevy::{
    prelude::{default, App, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use economy::plugin::EconomySimulationPlugin;
use ui::{render_ui, UIState};
use worldgen::create_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(EconomySimulationPlugin)
        .init_resource::<UIState>()
        .add_startup_system(create_world)
        .add_system(render_ui)
        .run();
}
