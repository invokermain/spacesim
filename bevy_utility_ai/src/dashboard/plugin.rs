use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Commands, Window};

pub struct UtilityAIDashboardPlugin;

impl Plugin for UtilityAIDashboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_new_window_system);
    }
}

fn create_new_window_system(mut commands: Commands) {
    commands.spawn(Window {
        title: "UtilityAIDashboard".to_owned(),
        ..Default::default()
    });
}
