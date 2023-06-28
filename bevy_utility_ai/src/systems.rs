pub mod make_decisions;
pub mod update_action;

use bevy::prelude::{Added, Commands, Component, Entity, Query};

use crate::ai_meta::AIMeta;

// TODO: add system that watches for component removal
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
