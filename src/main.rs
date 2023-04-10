mod common;
mod economy;
mod ui;
mod worldgen;

use bevy::{prelude::App, DefaultPlugins};
use bevy_egui::EguiPlugin;
use ui::{render_ui, ui_controls, UIState};
use worldgen::{create_companies, create_world};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UIState>()
        .add_startup_system(create_world)
        .add_startup_system(create_companies)
        .add_system(render_ui)
        .add_system(ui_controls)
        .run();
}
