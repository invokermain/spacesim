use bevy::app::FixedUpdate;
use bevy::prelude::Plugin;

use super::systems::orbit_planetary_body;

pub struct AstralBodySimulationPlugin;

impl Plugin for AstralBodySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, orbit_planetary_body);
    }
}
