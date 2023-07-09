pub mod ai_meta;
pub mod considerations;
pub mod decisions;
pub mod define_ai;
pub mod plugin;
pub mod response_curves;
pub mod systems;
pub mod utils;

#[cfg(feature = "dashboard")]
pub mod dashboard;

pub use bevy_utility_ai_macros::{input_system, targeted_input_system};

pub use crate::ai_meta::AIMeta;
use std::any::TypeId;
use std::fmt::Debug;

use crate::decisions::{Decision, Filter};
use bevy::{
    prelude::{Component, Entity, Resource},
    utils::{HashMap, HashSet},
};

#[derive(Debug)]
pub enum FilterDefinition {
    Any,
    Filtered(Vec<Vec<Filter>>),
}

impl FilterDefinition {
    pub fn merge(&mut self, other: &FilterDefinition) -> FilterDefinition {
        match (self, other) {
            (FilterDefinition::Any, FilterDefinition::Any) => FilterDefinition::Any,
            (FilterDefinition::Filtered(_), FilterDefinition::Any) => FilterDefinition::Any,
            (FilterDefinition::Any, FilterDefinition::Filtered(_)) => FilterDefinition::Any,
            (FilterDefinition::Filtered(x), FilterDefinition::Filtered(y)) => {
                let mut joined = x.clone();
                joined.extend(y.clone());
                FilterDefinition::Filtered(joined)
            }
        }
    }
}

pub struct TargetedInputRequirements {
    pub target_filter: FilterDefinition,
}

pub struct AIDefinition {
    /// The decisions that make up this AIDefinition
    pub decisions: Vec<Decision>,
    /// The simple inputs used for this AI, passed to AIDefinition on register.
    pub simple_inputs: HashSet<TypeId>,
    /// The targeted inputs used for this AI, passed to AIDefinition on register.
    pub targeted_inputs: HashMap<TypeId, TargetedInputRequirements>,
}

impl AIDefinition {
    pub fn requires_targeted_input(&self, input: &TypeId) -> bool {
        self.targeted_inputs.contains_key(input)
    }

    pub fn requires_simple_input(&self, input: &TypeId) -> bool {
        self.simple_inputs.contains(input)
    }

    pub fn get_targeted_input_requirements(
        &self,
        input: &TypeId,
    ) -> &TargetedInputRequirements {
        &self.targeted_inputs[input]
    }
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
