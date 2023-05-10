use std::any::TypeId;

use bevy::prelude::{
    debug, debug_span, AppTypeRegistry, Entity, EventWriter, Events, IntoSystemConfig,
    IntoSystemSetConfig, Plugin, Query, ReflectComponent, ReflectDefault, SystemSet, World,
};

use crate::AIMeta;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UtililityAISet {
    CalculateInputs,
    MakeDecisions,
    UpdateActions,
}

pub struct UpdateEntityAction {
    entity_id: Entity,
    old_action: Option<TypeId>,
    new_action: TypeId,
}

pub fn make_decisions(
    mut query: Query<(Entity, &mut AIMeta)>,
    mut event_writer: EventWriter<UpdateEntityAction>,
) {
    let span = debug_span!("Making Decisions");
    let _span = span.enter();

    for (entity_id, mut ai_meta) in query.iter_mut() {
        let span = debug_span!("", entity = entity_id.index());
        let _span = span.enter();
        let mut decisions = Vec::new();

        for decision in &ai_meta.decisions {
            let span = debug_span!("", action = decision.action_name);
            let _span = span.enter();

            let mut decision_score = 1.0;

            for consideration in &decision.considerations {
                let consideration_score = ai_meta.input_scores[&consideration.input];
                if consideration_score == f32::NEG_INFINITY {
                    debug!(
                        "It looks like input system for {} hasn't run, entity might have \
                         components missing?",
                        consideration.input_name
                    );
                } else {
                    debug!(
                        "Consideration score for {} is {:.2}",
                        consideration.input_name, consideration_score
                    );
                    decision_score *= consideration_score;
                }
            }

            decisions.push((
                decision.action,
                decision.action_name.clone(),
                decision_score,
            ));

            debug!("Final score: {:.2}", decision_score);
        }

        decisions.sort_by(|a, b| b.2.total_cmp(&a.2));
        let (top_action, top_action_name, top_score) = decisions.first().unwrap();

        // if we should keep doing what we are doing we can break loop early
        if Some(*top_action) == ai_meta.current_action {
            debug!(
                "Keeping same action '{}' with score {:.2}",
                top_action_name, top_score
            );
            ai_meta.current_action_score = *top_score;
            continue;
        } else {
            debug!(
                "Switching to new action '{}' with score {:.2}",
                top_action_name, top_score
            );

            // we should change what we are doing, do this in another system as it will
            // unfortunately require mut World access so isn't parallelisable.
            event_writer.send(UpdateEntityAction {
                entity_id,
                old_action: ai_meta.current_action,
                new_action: *top_action,
            });

            ai_meta.current_action = Some(*top_action);
            ai_meta.current_action_name = top_action_name.clone();
            ai_meta.current_action_score = *top_score;
        }
    }
}

pub fn update_action(world: &mut World) {
    let span = debug_span!("Updating Actions");
    let _span = span.enter();

    let type_registry = world.remove_resource::<AppTypeRegistry>().unwrap();

    let mut events = world
        .remove_resource::<Events<UpdateEntityAction>>()
        .unwrap();

    debug!("{} Events to process", events.len());

    {
        let registry_read = type_registry.read();

        for event in events.drain() {
            let UpdateEntityAction {
                entity_id,
                old_action,
                new_action,
            } = event;

            let span = debug_span!("", entity = entity_id.index());
            let _span = span.enter();

            if let Some(mut entity_mut) = world.get_entity_mut(entity_id) {
                if let Some(old_action) = old_action {
                    registry_read
                        .get(old_action)
                        .unwrap()
                        .data::<ReflectComponent>()
                        .unwrap()
                        .remove(&mut entity_mut);
                    debug!("Removed Action {:?}", old_action);
                }

                if let Some(registration) = registry_read.get(new_action) {
                    let reflect_default = registration.data::<ReflectDefault>().unwrap();
                    let reflect_component = registration.data::<ReflectComponent>().unwrap();
                    reflect_component.insert(&mut entity_mut, reflect_default.default().as_ref());
                    debug!("Added Action {:?}", new_action);
                } else {
                    panic!(
                        "An Action Component was not found in the type registry: {:?}",
                        new_action
                    )
                }
            } else {
                debug!("Unable to update Entity as it does not exist");
            }
        }
    }

    world.insert_resource(type_registry);
    world.insert_resource(events);
}

pub struct UtilityAIPlugin;

impl Plugin for UtilityAIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateEntityAction>()
            .add_system(make_decisions.in_set(UtililityAISet::MakeDecisions))
            .add_system(update_action.in_set(UtililityAISet::UpdateActions))
            .configure_set(UtililityAISet::CalculateInputs.before(UtililityAISet::MakeDecisions))
            .configure_set(UtililityAISet::MakeDecisions.before(UtililityAISet::UpdateActions));
    }
}
