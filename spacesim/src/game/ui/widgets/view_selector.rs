use bevy_egui::egui::{Context, TopBottomPanel};

use crate::game::ui::GameViewState;

pub(crate) fn widget_view_selector(ctx: &mut Context, current_view: &mut GameViewState) {
    TopBottomPanel::top("view_selector").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_view, GameViewState::System, "System");
            ui.selectable_value(current_view, GameViewState::Ship, "Ship");
            ui.selectable_value(current_view, GameViewState::Planet, "Planets");
            ui.selectable_value(current_view, GameViewState::Galaxy, "Galaxy");
        })
    });
}
