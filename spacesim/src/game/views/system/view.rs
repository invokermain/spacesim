use bevy::math::Vec3;
use bevy::prelude::{Color, Query, Transform, With};

use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::prelude::{Cap, DiscPainter, LinePainter};
use spacesim_simulation::common::marker_components::{IsPlanet, IsShip};
use spacesim_simulation::ships::components::SystemCoordinates;

use super::SCALING_FACTOR;

pub(crate) fn draw_planets(
    mut painter: ShapePainter,
    q_planets: Query<&SystemCoordinates, With<IsPlanet>>,
) {
    for coords in &q_planets {
        painter.transform = Transform::from_translation(coords.value / SCALING_FACTOR);
        // painter.thickness = 0.01;
        painter.color = Color::WHITE;
        painter.circle(0.1);
    }
}

pub(crate) fn draw_ships(
    mut painter: ShapePainter,
    q_ships: Query<&SystemCoordinates, With<IsShip>>,
) {
    for coords in &q_ships {
        painter.transform = Transform::from_translation(coords.value / SCALING_FACTOR);
        painter.thickness = 0.01;
        painter.color = Color::WHITE;
        painter.cap = Cap::None;
        painter.line(Vec3::new(-0.05, 0.05, 0.0), Vec3::new(0.05, -0.05, 0.0));
        painter.line(Vec3::new(-0.05, -0.05, 0.0), Vec3::new(0.05, 0.05, 0.0));
    }
}
