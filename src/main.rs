mod common;
mod economy;
mod ui;
mod worldgen;

use std::{time::Duration};

use bevy::{
    prelude::{App, IntoSystemConfig, PluginGroup, default},
    time::common_conditions::on_timer,
    DefaultPlugins, window::{WindowPlugin, Window},
};
use bevy_egui::EguiPlugin;
use economy::systems::{company_simulate, population_consumption};
use ui::{render_ui, ui_controls, UIState};
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
        .init_resource::<UIState>()
        .add_startup_system(create_world)
        .add_system(render_ui)
        .add_system(ui_controls)
        .add_system(company_simulate.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .add_system(population_consumption.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .run();
}
