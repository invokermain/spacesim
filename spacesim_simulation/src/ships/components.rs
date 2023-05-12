use bevy::prelude::{Component, Transform, Vec3};

#[derive(Component, Clone)]
pub struct SystemCoordinates {
    pub value: Vec3,
}

impl SystemCoordinates {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        Self {
            value: Vec3::new(x.into(), y.into(), z.into()),
        }
    }

    pub fn to_transform(&self) -> Transform {
        Transform::from_translation(self.value)
    }
}
