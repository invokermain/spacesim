use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{Entity, EventReader, KeyCode, ResMut, Resource, World},
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use crate::economy::components::{CommodityType, Market, Population};

#[derive(Default)]
enum View {
    #[default]
    Ship,
    Planet,
    System,
    Galaxy,
}

#[derive(Resource, Default)]
pub struct UIState {
    view: View,
}

impl UIState {
    pub fn next_view(&mut self) {
        match self.view {
            View::Ship => self.view = View::Planet,
            View::Planet => self.view = View::System,
            View::System => self.view = View::Galaxy,
            View::Galaxy => self.view = View::Ship,
        }
    }
}

pub fn ui_controls(mut key_evr: EventReader<KeyboardInput>, mut ui_state: ResMut<UIState>) {
    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if let Some(k) = ev.key_code {
                    match k {
                        KeyCode::Tab => ui_state.next_view(),
                        _ => (),
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn render_ui(world: &mut World) {
    let (egui_entity, _ctx) = world
        .query::<(Entity, &EguiContext)>()
        .get_single_mut(world)
        .unwrap();
    let mut egui_ctx = world.entity_mut(egui_entity).take::<EguiContext>().unwrap();

    egui::CentralPanel::default().show(egui_ctx.get_mut(), |ui| {
        let ui_state = world.get_resource::<UIState>().unwrap();
        match ui_state.view {
            View::Ship => {
                ui.heading("Ship View");
                ui.label("ship");
            }
            View::Planet => {
                ui.heading("Planet View");
                render_view_for_planet(world, ui);
            }
            View::System => {
                ui.heading("System View");
                ui.label("systems");
            }
            View::Galaxy => {
                ui.heading("Galaxy View");
                ui.label("wow");
            }
        }
    });

    world.entity_mut(egui_entity).insert(egui_ctx);
}

fn render_view_for_planet(world: &mut World, ui: &mut Ui) {
    let (_population, market) = world
        .query::<(&Population, &Market)>()
        .get_single(&world)
        .unwrap();

    ui.strong("MARKET");

    let table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::auto(), 3)
        .min_scrolled_height(0.0);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Commodity");
            });
            header.col(|ui| {
                ui.strong("Supply");
            });
            header.col(|ui| {
                ui.strong("Demand");
            });
        })
        .body(|mut body| {
            for commodity_type in CommodityType::iter() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{:?}", commodity_type));
                    });
                    row.col(|ui| {
                        ui.label(market.total_supply[commodity_type as usize].to_string());
                    });
                    row.col(|ui| {
                        ui.label(market.demand[commodity_type as usize].to_string());
                    });
                });
            }
        });
}
