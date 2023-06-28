#[allow(dead_code)]
pub mod app;

use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault, Vec2};

// Some Components
#[derive(Component)]
pub struct SomeData {
    pub val: f32,
}

#[derive(Component)]
pub struct SomeOtherData {
    pub val: f32,
}

#[derive(Component)]
pub struct Position {
    pub val: Vec2,
}

// Some Actions
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionOne {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionTwo {}

// AI Marker Components
#[derive(Component)]
pub struct AI {}

#[derive(Component)]
pub struct AI1 {}

#[derive(Component)]
pub struct AI2 {}

// ZST's for filtering
#[derive(Component)]
pub struct AA {}

#[derive(Component)]
pub struct BB {}

#[derive(Component)]
pub struct CC {}
