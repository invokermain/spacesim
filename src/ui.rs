mod planet_view;
mod query_layer;
mod render_structs;

use bevy::prelude::{Entity, Resource, World};
use bevy_egui::{
    egui::{self, TopBottomPanel},
    EguiContext,
};

use self::planet_view::{planet_view, PlanetViewState};

#[derive(Default, PartialEq)]
enum View {
    Ship,
    #[default]
    Planet,
    System,
    Galaxy,
}

#[derive(Resource, Default)]
pub struct UIState {
    view: View,
    planet_view_state: PlanetViewState,
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

pub fn render_ui(world: &mut World) {
    let (egui_entity, _ctx) = world
        .query::<(Entity, &EguiContext)>()
        .get_single_mut(world)
        .unwrap();
    let mut egui_ctx = world.entity_mut(egui_entity).take::<EguiContext>().unwrap();

    let mut ui_state = world.remove_resource::<UIState>().unwrap();

    TopBottomPanel::top("view_selector").show(egui_ctx.get_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut ui_state.view, View::System, "System");
            ui.selectable_value(&mut ui_state.view, View::Ship, "Ship");
            ui.selectable_value(&mut ui_state.view, View::Planet, "Planets");
            ui.selectable_value(&mut ui_state.view, View::Galaxy, "Galaxy");
        })
    });

    egui::CentralPanel::default().show(egui_ctx.get_mut(), |ui| match ui_state.view {
        View::Ship => {
            ui.heading("Ship View");
            ui.label("ship");
        }
        View::Planet => {
            ui.heading("Planet View");
            planet_view(ui, world, &mut ui_state.planet_view_state);
        }
        View::System => {
            ui.heading("System View");
            ui.label("systems");
        }
        View::Galaxy => {
            ui.heading("Galaxy View");
            ui.label("wow");
        }
    });

    world.insert_resource(ui_state);
    world.entity_mut(egui_entity).insert(egui_ctx);
}
