mod camera;
mod setup;
pub mod view;

use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

use crate::game::state::GameViewState;

use self::camera::{pan_orbit_camera, spawn_camera};
use self::setup::spawn_sun;
use self::view::view_system;

pub struct SystemViewPlugin;

pub(crate) const SCALING_FACTOR: f32 = 100_000_000.;

impl Plugin for SystemViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_camera.in_schedule(OnEnter(GameViewState::System)))
            .add_system(spawn_sun.in_schedule(OnEnter(GameViewState::System)))
            .add_system(view_system.in_set(OnUpdate(GameViewState::System)))
            .add_system(pan_orbit_camera.in_set(OnUpdate(GameViewState::System)));
    }
}
