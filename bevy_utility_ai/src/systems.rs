mod make_decisions;
mod update_action;
pub(crate) use make_decisions::make_decisions;
pub(crate) use update_action::update_action;

use std::any::TypeId;

use bevy::prelude::{Added, Commands, Component, Entity, Query, With, Without};

use crate::ai_meta::AIMeta;

pub struct UpdateEntityAction {
    entity_id: Entity,
    old_action: Option<TypeId>,
    new_action: TypeId,
    old_target: Option<Entity>,
    new_target: Option<Entity>,
}
//
// pub(crate) fn filter_input<F: Component>(
//     mut q_subject: Query<&mut AIMeta>,
//     q_target: Query<Entity, With<F>>,
// ) {
//     for mut ai_meta in q_subject.iter_mut() {
//         if ai_meta.ai_definition
//         for (entity_id, p0) in q_target.iter() {
//             if entity_id == subject_entity_id {
//                 continue;
//             }
//             let target = (p0,);
//             let score = { subject.0.val.distance(target.0.val) };
//             let entry = score_map.entry(entity_id).or_insert(f32::NEG_INFINITY);
//             *entry = score;
//         }
//     }
// }

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
