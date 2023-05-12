mod planet_view;
mod query_layer;
mod render_structs;
mod widgets;

use bevy::prelude::{Query, ResMut, Resource, State, States};
use bevy_egui::EguiContext;

use self::planet_view::PlanetViewState;
use self::widgets::widget_view_selector;

pub fn system_view(
    mut query: Query<&mut EguiContext>,
    mut res_game_view_state: ResMut<State<GameViewState>>,
) {
    let mut egui_ctx = query.single_mut();
    let game_view_state = res_game_view_state.as_mut().0;

    widget_view_selector(egui_ctx.get_mut(), &mut game_view_state);
}

// pub fn render_ui(world: &mut World) {
//     let (egui_entity, _ctx) = world
//         .query::<(Entity, &EguiContext)>()
//         .get_single_mut(world)
//         .unwrap();
//     let mut egui_ctx = world.entity_mut(egui_entity).take::<EguiContext>().unwrap();

//     let mut view = world.remove_resource::<GameView>().unwrap();

//     TopBottomPanel::top("view_selector").show(egui_ctx.get_mut(), |ui| {
//         ui.horizontal(|ui| {
//             ui.selectable_value(&mut ui_state.view, View::System, "System");
//             ui.selectable_value(&mut ui_state.view, View::Ship, "Ship");
//             ui.selectable_value(&mut ui_state.view, View::Planet, "Planets");
//             ui.selectable_value(&mut ui_state.view, View::Galaxy, "Galaxy");
//         })
//     });

//     egui::CentralPanel::default().show(egui_ctx.get_mut(), |ui| match ui_state.view {
//         View::Ship => {
//             ui.heading("Ship View");
//             ui.label("ship");
//         }
//         View::Planet => {
//             ui.heading("Planet View");
//             planet_view(ui, world, &mut ui_state.planet_view_state);
//         }
//         View::System => {
//             ui.heading("System View");
//             ui.label("systems");
//         }
//         View::Galaxy => {
//             ui.heading("Galaxy View");
//             ui.label("wow");
//         }
//     });

//     world.insert_resource(view);
//     world.entity_mut(egui_entity).insert(egui_ctx);
// }
