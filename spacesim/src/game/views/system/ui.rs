use bevy::prelude::{Query, ResMut, State, With};
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;

use crate::game::state::GameViewState;
use crate::game::ui::widgets::view_selector::widget_view_selector;

pub fn system_ui(
    mut query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut res_game_view_state: ResMut<State<GameViewState>>,
) {
    let mut egui_ctx = query.single_mut();
    let game_view_state = res_game_view_state.as_mut();

    widget_view_selector(egui_ctx.get_mut(), &mut game_view_state.0);
}
