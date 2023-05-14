mod camera;
mod setup;
mod ui;
pub mod view;

use bevy::prelude::{
    Camera, Entity, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
    Query, Res, Resource,
};

use crate::game::state::GameViewState;

use self::camera::{pan_orbit_camera, spawn_camera};
use self::setup::{spawn_axes, spawn_sun};
use self::ui::system_ui;
use self::view::view_system;

pub struct SystemViewPlugin;

pub(crate) const SCALING_FACTOR: f32 = 100_000_000.;

/// Note these shouldn't be accessed before startup systems have run succesfully
#[derive(Resource)]
pub(crate) struct SystemViewHandles {
    pub(crate) camera: Entity,
    pub(crate) axes_lines: (Entity, Entity, Entity), // (x, y, z)
    pub(crate) sun: Entity,
}

impl Default for SystemViewHandles {
    fn default() -> Self {
        Self {
            camera: Entity::from_raw(0),
            axes_lines: (
                Entity::from_raw(0),
                Entity::from_raw(0),
                Entity::from_raw(0),
            ),
            sun: Entity::from_raw(0),
        }
    }
}

fn toggle_camera<const V: bool>(
    mut q_camera: Query<&mut Camera>,
    r_handles: Res<SystemViewHandles>,
) {
    if let Ok(mut camera) = q_camera.get_mut(r_handles.camera) {
        camera.is_active = V;
    }
}

impl Plugin for SystemViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SystemViewHandles>()
            .add_startup_systems((spawn_camera, spawn_sun, spawn_axes))
            .add_system(toggle_camera::<true>.in_schedule(OnEnter(GameViewState::System)))
            .add_system(view_system.in_set(OnUpdate(GameViewState::System)))
            .add_system(pan_orbit_camera.in_set(OnUpdate(GameViewState::System)))
            .add_system(system_ui.in_set(OnUpdate(GameViewState::System)))
            .add_system(toggle_camera::<false>.in_schedule(OnExit(GameViewState::System)));
    }
}
