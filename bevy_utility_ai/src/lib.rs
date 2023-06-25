pub mod ai_meta;
pub mod considerations;
pub mod decisions;
pub mod define_ai;
pub mod plugin;
pub mod response_curves;
pub mod systems;

pub use bevy_utility_ai_macros::{input_system, targeted_input_system};

pub use crate::ai_meta::AIMeta;
use std::any::TypeId;

use crate::decisions::Decision;
use crate::define_ai::TargetedInputRequirements;
use bevy::{
    prelude::{Component, Entity, Resource},
    utils::{HashMap, HashSet},
};

pub struct AIDefinition {
    /// The decisions that make up this AIDefinition
    pub decisions: Vec<Decision>,
    /// The simple inputs used for this AI, passed to AIDefinition on register.
    simple_inputs: HashSet<usize>,
    /// The targeted inputs used for this AI, passed to AIDefinition on register.
    targeted_inputs: HashMap<usize, TargetedInputRequirements>,
}

impl AIDefinition {
    pub fn requires_targeted_input(&self, input: &usize) -> bool {
        // TODO: doesn't feel great that we have to do two lookups, maybe a design smell
        self.simple_inputs.contains(input) || self.targeted_inputs.contains_key(input)
    }

    pub fn get_targeted_input_requirements(
        &self,
        input: &usize,
    ) -> &TargetedInputRequirements {
        &self.targeted_inputs[&input]
    }
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    pub map: HashMap<TypeId, AIDefinition>,
}

/// This Resource contains a map of `inclusive_targeted_filter_input` keys to a set of entities.
/// Targeted input systems can look up the relevant filters using this resource when they run.
/// The system will lookup what filters it will respect against the entity's associated AIDefinition
#[derive(Resource, Default)]
pub struct AITargetEntitySets {
    entity_set_map: HashMap<usize, HashSet<Entity>>,
}

impl AITargetEntitySets {
    pub fn get_intersection_of<'a>(
        &self,
        entity_sets: impl IntoIterator<Item = &'a usize>,
    ) -> Vec<Entity> {
        entity_sets
            .into_iter()
            .filter_map(|set_key| self.entity_set_map.get(set_key))
            .cloned()
            .reduce(|acc, e| acc.intersection(&e).cloned().collect())
            .unwrap()
            .into_iter()
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

#[cfg(test)]
mod tests {
    use crate::AITargetEntitySets;
    use bevy::prelude::Entity;
    use bevy::utils::hashbrown::HashMap;
    use bevy::utils::HashSet;

    #[test]
    fn aitarget_entity_sets_get_intersection_of() {
        let x = AITargetEntitySets {
            entity_set_map: HashMap::from_iter(vec![
                (
                    1,
                    HashSet::from_iter(vec![Entity::from_raw(0), Entity::from_raw(1)]),
                ),
                (
                    2,
                    HashSet::from_iter(vec![Entity::from_raw(1), Entity::from_raw(2)]),
                ),
            ]),
        };

        assert_eq!(
            x.get_intersection_of(vec![&1]),
            vec![Entity::from_raw(0), Entity::from_raw(1)]
        );
        assert_eq!(
            x.get_intersection_of(vec![&1, &2]),
            vec![Entity::from_raw(1)]
        );
    }
}
