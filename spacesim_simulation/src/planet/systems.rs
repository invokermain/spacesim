use std::f32::consts::PI;

use bevy::prelude::{Quat, Query, Vec3, With};

use crate::common::marker_components::IsPlanet;
use crate::ships::components::SystemCoordinates;

pub(crate) fn orbit_planetary_body(
    mut q_planet: Query<&mut SystemCoordinates, With<IsPlanet>>,
) {
    for mut coords in q_planet.iter_mut() {
        let mut transform = coords.to_transform();
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI * 0.001));
        *coords = transform.into();
    }
}
