mod common;
mod economy;
mod ui;
mod worldgen;

use bevy::{prelude::App, DefaultPlugins};
use bevy_egui::EguiPlugin;
use economy::{
    events::CommodityProducedEvent, market::market_supply_update, systems::company_simulate,
};
use ui::{render_ui, ui_controls, UIState};
use worldgen::create_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UIState>()
        .add_event::<CommodityProducedEvent>()
        .add_startup_system(create_world)
        .add_system(render_ui)
        .add_system(ui_controls)
        .add_system(company_simulate)
        .add_system(market_supply_update)
        .run();
}
