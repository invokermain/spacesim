pub mod ai_meta;
pub mod considerations;
pub mod define_ai;
pub mod plugin;
pub mod response_curves;
pub mod systems;
pub use bevy_utility_ai_macros::{input_system, targeted_input_system};

pub use crate::ai_meta::AIMeta;
use crate::considerations::Consideration;
use std::any::TypeId;

use bevy::{
    prelude::{Component, Entity, Resource},
    utils::{HashMap, HashSet},
};

pub struct AIDefinition {
    pub decisions: Vec<Decision>,
    pub required_inputs: HashSet<usize>,
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    pub map: HashMap<TypeId, AIDefinition>,
}

/// A component to hold the Target entity ID
#[derive(Component)]
pub struct ActionTarget {
    pub target: Entity,
}

pub struct Decision {
    pub action_name: String,
    pub action: TypeId,
    pub simple_considerations: Vec<Consideration>,
    pub targeted_considerations: Vec<Consideration>,
    pub is_targeted: bool,
}
