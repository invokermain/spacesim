use bevy::app::{CoreSchedule, IntoSystemAppConfig};
use bevy::prelude::Plugin;

use super::systems::orbit_planetary_body;

pub struct AstralBodySimulationPlugin;

impl Plugin for AstralBodySimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(orbit_planetary_body.in_schedule(CoreSchedule::FixedUpdate));
    }
}
