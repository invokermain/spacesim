use bevy::prelude::{Component, Entity};
use big_brain::prelude::ActionBuilder;

#[derive(Component, Clone, Copy)]
pub enum ShipPersonality {
    Trader,
}

#[derive(Debug, Component, Clone, Copy, ActionBuilder)]
pub struct Travelling {
    destination: Entity,
}

#[derive(Debug, Component, Clone, Copy, ActionBuilder)]
pub struct Docked {
    at: Entity,
}
