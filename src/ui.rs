mod planet_view;
mod query_layer;
mod render_structs;

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{Entity, EventReader, KeyCode, ResMut, Resource, World},
};
use bevy_egui::{egui, EguiContext};

use self::planet_view::{planet_view, PlanetViewState};

#[derive(Default)]
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
                planet_view(ui, world);
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
