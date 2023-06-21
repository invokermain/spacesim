use bevy::app::App;
use bevy::log::LogPlugin;

pub fn test_app() -> App {
    let mut app = App::new();
    app.add_plugin(LogPlugin {
        filter: "wgpu=error".into(),
        level: bevy::log::Level::DEBUG,
    });
    app
}
