use bevy::math::Quat;
use bevy::prelude::{Color, Query, Vec2, With};
use std::f32::consts::PI;

use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::prelude::{Cap, DiscPainter, LinePainter, RectPainter};
use spacesim_simulation::common::marker_components::{IsPlanet, IsShip};
use spacesim_simulation::ships::components::SystemCoordinates;

use super::SCALING_FACTOR;

pub(crate) fn draw_planets(
    mut painter: ShapePainter,
    q_planets: Query<&SystemCoordinates, With<IsPlanet>>,
) {
    for coords in &q_planets {
        painter.set_translation(coords.value / SCALING_FACTOR);
        painter.color = Color::WHITE;
        painter.circle(0.1);
    }
}

pub(crate) fn draw_ships(
    mut painter: ShapePainter,
    q_ships: Query<&SystemCoordinates, With<IsShip>>,
) {
    for coords in &q_ships {
        painter.set_translation(coords.value / SCALING_FACTOR);
        painter.color = Color::WHITE;
        painter.cap = Cap::None;

        painter.set_rotation(Quat::from_rotation_x(-0.25 * PI));
        painter.rect(Vec2::new(0.01, 0.10));

        painter.set_rotation(Quat::from_rotation_x(0.25 * PI));
        painter.rect(Vec2::new(0.01, 0.10));
    }
}
