mod state;
mod ui;
mod views;

use bevy::prelude::Plugin;
use bevy_egui::EguiPlugin;

use self::state::GameViewState;
use self::views::planet::PlanetViewPlugin;
use self::views::system::SystemViewPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin)
            .add_state::<GameViewState>()
            .add_plugin(SystemViewPlugin)
            .add_plugin(PlanetViewPlugin);
    }
}
