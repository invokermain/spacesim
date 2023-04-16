mod common;
mod economy;
mod ui;
mod worldgen;

use std::time::Duration;

use bevy::{
    prelude::{App, IntoSystemConfig},
    time::common_conditions::on_timer,
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use economy::systems::{company_simulate, population_consumption};
use ui::{render_ui, ui_controls, UIState};
use worldgen::create_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UIState>()
        .add_startup_system(create_world)
        .add_system(render_ui)
        .add_system(ui_controls)
        .add_system(company_simulate.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .add_system(population_consumption.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .run();
}
