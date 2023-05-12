mod ui;

use bevy::prelude::{IntoSystemConfig, Plugin, Res};
use bevy_egui::EguiPlugin;

use self::ui::{system_view, GameView};

pub struct GamePlugin;

fn game_view_is_system(game_view: Res<GameView>) -> bool {
    matches!(game_view.as_ref(), GameView::System)
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<GameView>()
            .add_system(system_view.run_if(game_view_is_system));
    }
}
