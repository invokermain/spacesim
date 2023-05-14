use bevy_egui::egui::{Context, TopBottomPanel};

use crate::game::state::GameViewState;

pub fn widget_view_selector(ctx: &mut Context, current_view: &mut GameViewState) {
    TopBottomPanel::top("view_selector").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_view, GameViewState::System, "System");
            ui.selectable_value(current_view, GameViewState::Planet, "Planets");
            ui.label("Ship (soon)");
            ui.label("Galaxy (soon soon)");
        })
    });
}
