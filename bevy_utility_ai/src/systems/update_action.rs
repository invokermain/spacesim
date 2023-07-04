use bevy::prelude::{
    debug, debug_span, AppTypeRegistry, Entity, Event, Events, ReflectComponent,
    ReflectDefault, World,
};
use std::any::TypeId;

use crate::ActionTarget;

pub(crate) fn update_actions_sys(world: &mut World) {
    let _span = debug_span!("Updating Actions").entered();

    let type_registry = world.remove_resource::<AppTypeRegistry>().unwrap();

    let mut events = world
        .remove_resource::<Events<UpdateEntityActionInternal>>()
        .unwrap();

    if !events.is_empty() {
        debug!("{} Events to process", events.len());
        let registry_read = type_registry.read();

        for event in events.drain() {
            let UpdateEntityActionInternal {
                entity_id,
                old_action,
                new_action,
                old_target,
                new_target,
            } = event;

            let _span = debug_span!("", entity = entity_id.index()).entered();

            if let Some(mut entity_mut) = world.get_entity_mut(entity_id) {
                // Update the action on the entity
                if old_action != Some(new_action) {
                    // Remove the old action component
                    if let Some(old_action) = old_action {
                        registry_read
                            .get(old_action)
                            .unwrap()
                            .data::<ReflectComponent>()
                            .unwrap()
                            .remove(&mut entity_mut);
                        debug!("Removed Action {:?}", old_action);
                    }

                    // Add the new action component
                    if let Some(registration) = registry_read.get(new_action) {
                        let reflect_default = registration.data::<ReflectDefault>().unwrap();
                        let reflect_component =
                            registration.data::<ReflectComponent>().unwrap();
                        reflect_component
                            .insert(&mut entity_mut, reflect_default.default().as_ref());
                        debug!("Added Action {:?}", new_action);
                    } else {
                        panic!(
                            "An Action Component was not found in the type registry: {:?}",
                            new_action
                        )
                    }
                } else {
                    debug!("Action is the same as current action");
                }

                // Update the target on the entity
                if old_target != new_target {
                    if entity_mut.contains::<ActionTarget>() {
                        entity_mut.remove::<ActionTarget>();
                        debug!("Removed Target");
                    }

                    if let Some(target) = new_target {
                        entity_mut.insert(ActionTarget { target });
                        debug!("Added Target {:?}", target);
                    }
                }
            } else {
                debug!("Unable to update Entity as it does not exist");
            }
        }
    }

    world.insert_resource(type_registry);
    world.insert_resource(events);
}

#[derive(Event)]
pub(crate) struct UpdateEntityActionInternal {
    pub(crate) entity_id: Entity,
    pub(crate) old_action: Option<TypeId>,
    pub(crate) new_action: TypeId,
    pub(crate) old_target: Option<Entity>,
    pub(crate) new_target: Option<Entity>,
}
