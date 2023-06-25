use bevy::prelude::{Commands, Plugin, Window};

pub struct DebuggerPlugin;

impl Plugin for DebuggerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_new_window_system);
    }
}

fn create_new_window_system(mut commands: Commands) {
    let debugger_window_id = commands
        .spawn(Window {
            title: "SpaceSim Debugger".to_owned(),
            ..Default::default()
        })
        .id();
}
