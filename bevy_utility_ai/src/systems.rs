mod make_decisions;
mod update_action;
pub(crate) use make_decisions::make_decisions;
pub(crate) use update_action::update_action;

use std::any::TypeId;

use bevy::prelude::{Added, Commands, Component, Entity, Query, With, Without};
use bevy::utils::hashbrown::HashSet;

use crate::ai_meta::AIMeta;

pub struct UpdateEntityAction {
    entity_id: Entity,
    old_action: Option<TypeId>,
    new_action: TypeId,
    old_target: Option<Entity>,
    new_target: Option<Entity>,
}

pub(crate) fn inclusive_filter_input<F: Component>(
    mut q_subject: Query<&mut AIMeta>,
    q_target: Query<Entity, With<F>>,
) {
    for mut ai_meta in q_subject.iter_mut() {
        match &mut ai_meta.valid_target_set {
            // None implies all entities are valid targets
            None => {
                ai_meta.valid_target_set =
                    Some(HashSet::from_iter(q_target.iter().collect::<Vec<Entity>>()))
            }
            // Some implies only entities in set are valid targets
            Some(target_set) => {
                target_set.drain_filter(|&entity| !q_target.contains(entity));
            }
        }
    }
}

pub(crate) fn ensure_entity_has_ai_meta<T: Component>(
    mut commmads: Commands,
    query: Query<(Entity, Option<&AIMeta>), Added<T>>,
) {
    for (entity, ai_meta) in &query {
        if ai_meta.is_none() {
            commmads.entity(entity).insert(AIMeta::new::<T>());
        }
    }
}
