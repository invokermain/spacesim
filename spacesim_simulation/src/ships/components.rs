use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct SystemCoordinates {
    value: Vec3,
}

impl SystemCoordinates {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        Self {
            value: Vec3::new(x.into(), y.into(), z.into()),
        }
    }
}
