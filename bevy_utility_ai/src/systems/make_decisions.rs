use crate::systems::UpdateEntityAction;
use crate::{AIDefinitions, AIMeta, Decision};
use bevy::log::{debug, debug_span};
use bevy::prelude::{Entity, EventWriter, Query, Res};
use bevy::utils::HashMap;

pub(crate) fn make_decisions(
    mut query: Query<(Entity, &mut AIMeta)>,
    mut event_writer: EventWriter<UpdateEntityAction>,
    ai_definitions: Res<AIDefinitions>,
) {
    let _span = debug_span!("Making Decisions").entered();

    for (entity_id, mut ai_meta) in query.iter_mut() {
        let ai_definition = &ai_definitions.map[&ai_meta.ai_definition];

        let _span = debug_span!("", entity = entity_id.index()).entered();
        let mut evaluated_decisions = Vec::new();

        for (idx, decision) in ai_definition.decisions.iter().enumerate() {
            let span = debug_span!("", action = decision.action_name);
            let _span = span.enter();

            let mut decision_score = 1.0;

            // consider non-targeted considerations
            for consideration in &decision.simple_considerations {
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
                let score_map = ai_meta.targeted_input_scores.get(&consideration.input);
                if score_map.is_none() {
                    debug!(
                        "No scores where registered for targeted input system {}",
                        consideration.input_name
                    );
                };
                for (&target_entity, &consideration_input_score) in score_map.unwrap() {
                    let consideration_score = consideration
                        .response_curve
                        .transform(consideration_input_score)
                        .clamp(0.0, 1.0);
                    debug!(
                        "Consideration score for targeted system {} and entity {:?} is {:.2} (raw {:.2})",
                        consideration.input_name, target_entity, consideration_score, consideration_input_score
                    );

                    *targeted_scores
                        .entry(target_entity)
                        .or_insert(decision_score) *= consideration_score;
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
