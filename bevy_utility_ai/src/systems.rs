pub mod make_decisions;
pub mod update_action;

use bevy::prelude::{Added, Commands, Component, Entity, Query, ResMut};

use crate::ai_meta::AIMeta;
use crate::AITargetEntitySets;

// TODO: add system that watches for component removal
pub(crate) fn inclusive_targeted_filter_input<F: Component>(
    q_added: Query<Entity, Added<F>>,
    mut res_target_filter_sets: ResMut<AITargetEntitySets>,
) {
    let key = inclusive_targeted_filter_input::<F> as usize;
    for added_entity in q_added.iter() {
        res_target_filter_sets.insert(key, added_entity);
    }
}

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

#[cfg(test)]
mod tests {
    use crate::systems::inclusive_targeted_filter_input;
    use crate::{AIMeta, AITargetEntitySets};
    use bevy::app::App;
    use bevy::prelude::Component;

    #[test]
    fn inclusive_filter_input_includes_valid_target() {
        #[derive(Component)]
        struct FilterTarget {}

        #[derive(Component)]
        struct OtherComponent {}

        #[derive(Component)]
        struct AIMarker {}

        let mut app = App::new();
        app.init_resource::<AITargetEntitySets>();

        app.add_system(inclusive_targeted_filter_input::<FilterTarget>);

        app.world.spawn((AIMarker {}, AIMeta::new::<AIMarker>()));
        let entity_valid_target = app.world.spawn((FilterTarget {},)).id();
        let entity_invalid_target = app.world.spawn((OtherComponent {},)).id();

        app.update();

        let ai_target_entity_sets = app.world.get_resource::<AITargetEntitySets>().unwrap();
        let entity_set = ai_target_entity_sets
            .entity_set_map
            .get(&(inclusive_targeted_filter_input::<FilterTarget> as usize))
            .unwrap();

        assert!(!entity_set.is_empty());
        assert!(entity_set.contains(&entity_valid_target));
        assert!(!entity_set.contains(&entity_invalid_target));
    }
}
