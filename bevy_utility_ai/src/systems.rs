use std::any::TypeId;

use bevy::prelude::{
    debug, debug_span, AppTypeRegistry, Entity, EventWriter, Events, IntoSystemConfig,
    IntoSystemSetConfig, Plugin, Query, ReflectComponent, ReflectDefault, Res, SystemSet,
    World,
};
use bevy::utils::HashMap;

use crate::ai_meta::AIMeta;
use crate::{AIDefinitions, ActionTarget, Decision};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UtililityAISet {
    SelectTargets,
    CalculateInputs,
    MakeDecisions,
    UpdateActions,
}

pub struct UpdateEntityAction {
    entity_id: Entity,
    old_action: Option<TypeId>,
    new_action: TypeId,
    old_target: Option<Entity>,
    new_target: Option<Entity>,
}

pub fn make_decisions(
    mut query: Query<(Entity, &mut AIMeta)>,
    mut event_writer: EventWriter<UpdateEntityAction>,
    ai_definitions: Res<AIDefinitions>,
) {
    let span = debug_span!("Making Decisions");
    let _span = span.enter();

    for (entity_id, mut ai_meta) in query.iter_mut() {
        let ai_definition = &ai_definitions.map[&ai_meta.ai_definition];

        let span = debug_span!("", entity = entity_id.index());
        let _span = span.enter();
        let mut evaluated_decisions = Vec::new();

        for (idx, decision) in ai_definition.decisions.iter().enumerate() {
            let span = debug_span!("", action = decision.action_name);
            let _span = span.enter();

            let mut decision_score = 1.0;

            // consider non-targeted considerations
            for consideration in &decision.considerations {
                let consideration_input_score = *ai_meta
                    .input_scores
                    .get(&consideration.input)
                    .unwrap_or(&f32::NEG_INFINITY);
                if consideration_input_score == f32::NEG_INFINITY {
                    debug!(
                        "It looks like input system for {} hasn't run, entity might have \
                         components missing?",
                        consideration.input_name
                    );
                } else {
                    let consideration_score = consideration
                        .response_curve
                        .transform(consideration_input_score)
                        .clamp(0.0, 1.0);
                    debug!(
                        "Consideration score for {} is {:.2} (raw {:.2})",
                        consideration.input_name,
                        consideration_score,
                        consideration_input_score
                    );
                    decision_score *= consideration_score;
                }
            }

            if !decision.is_targeted {
                evaluated_decisions.push((idx, None, decision_score));
                debug!("Decision {} scored {:.2}", idx, decision_score);
                continue;
            }

            let mut targeted_scores = HashMap::new();

            // consider targeted considerations
            for consideration in &decision.targeted_considerations {
                let targets = ai_meta.targeted_input_targets[&consideration.input].clone();
                for entity_id in targets {
                    let consideration_input_score = *ai_meta
                        .targeted_input_scores
                        .get(&(consideration.input, entity_id))
                        .unwrap_or(&f32::NEG_INFINITY);
                    if consideration_input_score == f32::NEG_INFINITY {
                        debug!(
                            "It looks like targeted input system {} for entity {:?} hasn't run, \
                            entity might have components missing?",
                            consideration.input_name, entity_id
                        );
                    } else {
                        let consideration_score = consideration
                            .response_curve
                            .transform(consideration_input_score)
                            .clamp(0.0, 1.0);
                        debug!(
                            "Consideration score for targeted system {} and entity {:?} is {:.2} (raw {:.2})",
                            consideration.input_name, entity_id, consideration_score, consideration_input_score
                        );

                        *targeted_scores.entry(entity_id).or_insert(decision_score) *=
                            consideration_score;
                    }
                }
            }

            for (entity, targeted_decision_score) in targeted_scores {
                evaluated_decisions.push((idx, Some(entity), targeted_decision_score));
                debug!(
                    "Decision {} for entity {:?} scored {:.2}",
                    idx, entity, targeted_decision_score
                );
            }
        }

        // pick best decision
        evaluated_decisions.sort_by(|a, b| b.2.total_cmp(&a.2));

        let (decision_idx, target, score) = evaluated_decisions.first().unwrap();
        let Decision {
            action_name,
            action,
            is_targeted,
            ..
        } = &ai_definition.decisions[*decision_idx];

        let keep_current_action = Some(*action) == ai_meta.current_action;
        let keep_current_target = *target == ai_meta.current_target;

        if keep_current_action && keep_current_target {
            // Scenario 1: Same Action, keep same target (which can be None)
            if *is_targeted {
                debug!(
                    "Keeping same action '{}' targeting {:?} with score {:.2}",
                    action_name, target, score
                );
            } else {
                debug!(
                    "Keeping same action '{}' with score {:.2}",
                    action_name, score
                );
            }
            ai_meta.current_action_score = *score;
            continue;
        } else {
            if !keep_current_action {
                // Scenario 3: New action
                if *is_targeted {
                    debug!(
                        "Switching to new action '{}' targeting {:?} with score {:.2}",
                        action_name, target, score
                    );
                } else {
                    debug!(
                        "Switching to new action '{}' with score {:.2}",
                        action_name, score
                    );
                }
            } else if !keep_current_target {
                // Scenario 2:  Same Action (targeted), switch to new target
                debug_assert!(is_targeted, "This Action {} isn't targeted", action_name);
                debug!(
                    "Keeping Action {} but switching target to {:?} with score {:.2}",
                    action_name, target, score
                );
            } else {
                panic!("How did we get here?");
            }

            // Change our currection action, we do this in another system as it will
            // unfortunately require mut World access so isn't parallelisable.
            event_writer.send(UpdateEntityAction {
                entity_id,
                old_action: ai_meta.current_action,
                new_action: *action,
                old_target: ai_meta.current_target,
                new_target: *target,
            });

            ai_meta.current_action = Some(*action);
            ai_meta.current_action_name = action_name.clone();
            ai_meta.current_action_score = *score;
            ai_meta.current_target = *target;
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
                old_target,
                new_target,
            } = event;

            let span = debug_span!("", entity = entity_id.index());
            let _span = span.enter();

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
                    debug!("Unable to update Entity as it does not exist");
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
            .configure_set(
                UtililityAISet::CalculateInputs.before(UtililityAISet::MakeDecisions),
            )
            .configure_set(
                UtililityAISet::MakeDecisions.before(UtililityAISet::UpdateActions),
            );
    }
}
