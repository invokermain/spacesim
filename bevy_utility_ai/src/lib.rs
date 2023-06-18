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
    /// map of targeted_input_system key to set of target filter set keys, seeAITargetEntitySets
    pub targeted_input_filter_sets: HashMap<usize, Vec<usize>>,
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    pub map: HashMap<TypeId, AIDefinition>,
}

#[derive(Resource, Default)]
pub struct AITargetEntitySets {
    // map of filter_system key to entity set
    entity_set_map: HashMap<usize, HashSet<Entity>>,
}

impl AITargetEntitySets {
    pub fn get(&self, entity_set: usize) -> Vec<Entity> {
        self.entity_set_map
            .get(&entity_set)
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }

    pub fn insert(&mut self, filter_system_key: usize, entity: Entity) {
        let entry = self.entity_set_map.entry(filter_system_key).or_default();
        entry.insert(entity);
    }

    pub fn remove(&mut self, filter_system_key: usize, entity: Entity) {
        let entry = self.entity_set_map.entry(filter_system_key).or_default();
        entry.remove(&entity);
    }
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
