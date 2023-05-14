mod ui;

use bevy::prelude::{Entity, IntoSystemConfig, OnUpdate, Plugin, Resource};

use crate::game::state::GameViewState;

use self::ui::planet_ui;

#[derive(Resource, Default)]
pub struct PlanetViewState {
    selected_planet: Option<Entity>,
}

pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PlanetViewState>()
            .add_system(planet_ui.in_set(OnUpdate(GameViewState::Planet)));
    }
}
