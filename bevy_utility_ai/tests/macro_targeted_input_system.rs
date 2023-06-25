mod common;

use crate::common::SomeOtherData;
use bevy::prelude::{Component, Query};
use bevy::utils::hashbrown::HashSet;
use bevy::{app::App, utils::HashMap};
use bevy_utility_ai::{AIDefinition, AIDefinitions, AIMeta, AITargetEntitySets};
use bevy_utility_ai_macros::targeted_input_system;
use common::{SomeData, AI};
use std::any::TypeId;

fn test_app() -> App {
    let mut app = App::new();
    app.init_resource::<AIDefinitions>();
    app.init_resource::<AITargetEntitySets>();
    app
}

#[test]
fn simple_targeted_input_system_produces_valid_system() {
    #[targeted_input_system]
    fn simple_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = App::new();
    app.add_system(simple_targeted_input);
}

#[test]
fn targeted_input_system_produces_valid_system() {
    #[targeted_input_system]
    fn targeted_input(subject: (&SomeOtherData,), target: (&SomeData,)) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = App::new();
    app.add_system(targeted_input);
}

#[test]
fn simple_targeted_input_system_updates_aimeta_inputs() {
    #[targeted_input_system]
    fn simple_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = test_app();

    app.add_system(simple_targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![simple_targeted_input as usize]),
            targeted_input_filter_sets: HashMap::new(),
        },
    );

    let subject_entity_id = app.world.spawn((AI {}, AIMeta::new::<AI>())).id();
    let target_entity_id = app.world.spawn(SomeData { val: 0.25 }).id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(subject_entity_id).unwrap();

    assert_eq!(ai_meta.targeted_input_scores.len(), 1);
    assert!(ai_meta
        .targeted_input_scores
        .contains_key(&(simple_targeted_input as usize)));
    assert_eq!(
        ai_meta.targeted_input_scores[&(simple_targeted_input as usize)][&target_entity_id],
        0.25
    );
}

#[test]
fn targeted_input_system_updates_aimeta_inputs() {
    #[targeted_input_system]
    fn targeted_input(subject: (&SomeOtherData,), target: (&SomeData,)) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = test_app();

    app.add_system(targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![targeted_input as usize]),
            targeted_input_filter_sets: HashMap::new(),
        },
    );

    let subject_entity_id = app
        .world
        .spawn((SomeOtherData { val: 0.75 }, AI {}, AIMeta::new::<AI>()))
        .id();
    let target_entity_id = app.world.spawn(SomeData { val: 0.25 }).id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(subject_entity_id).unwrap();

    assert_eq!(ai_meta.targeted_input_scores.len(), 1);
    assert!(ai_meta
        .targeted_input_scores
        .contains_key(&(targeted_input as usize)));
    assert_eq!(
        ai_meta.targeted_input_scores[&(targeted_input as usize)][&target_entity_id],
        0.5
    );
}

#[test]
fn simple_targeted_input_system_respects_filter_set() {
    #[targeted_input_system]
    fn simple_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = test_app();
    app.add_system(simple_targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![simple_targeted_input as usize]),
            targeted_input_filter_sets: HashMap::from_iter(vec![(
                simple_targeted_input as usize,
                vec![1],
            )]),
        },
    );

    // spawn some entities
    let entity_subject = app.world.spawn((AI {}, AIMeta::new::<AI>())).id();
    let entity_ignore = app.world.spawn(SomeData { val: 0.25 }).id();
    let entity_target = app.world.spawn(SomeData { val: 0.75 }).id();

    let mut ai_target_entity_sets = app.world.resource_mut::<AITargetEntitySets>();
    ai_target_entity_sets.insert(1, entity_target);

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_subject).unwrap();

    assert_eq!(ai_meta.targeted_input_scores.len(), 1);
    assert!(ai_meta
        .targeted_input_scores
        .contains_key(&(simple_targeted_input as usize)));
    assert_eq!(
        ai_meta.targeted_input_scores[&(simple_targeted_input as usize)][&entity_target],
        0.75
    );
    assert!(
        !ai_meta.targeted_input_scores[&(simple_targeted_input as usize)]
            .contains_key(&entity_ignore)
    );
}

#[test]
fn scrap() {
    fn targeted_input(
        mut q_subject: bevy::prelude::Query<(
            bevy::prelude::Entity,
            &mut bevy_utility_ai::AIMeta,
            &SomeOtherData,
        )>,
        q_target: bevy::prelude::Query<(bevy::prelude::Entity, &SomeData)>,
        res_ai_definitions: bevy::prelude::Res<bevy_utility_ai::AIDefinitions>,
        res_ai_target_entity_sets: bevy::prelude::Res<bevy_utility_ai::AITargetEntitySets>,
    ) {
        let _span =
            bevy::prelude::debug_span!("Calculating Targeted Input", input = "targeted_input")
                .entered();
        let key = targeted_input as usize;
        for (subject_entity_id, mut ai_meta, p0) in q_subject.iter_mut() {
            let _span =
                bevy::prelude::debug_span!("", entity = subject_entity_id.index()).entered();
            let ai_definition = res_ai_definitions.map.get(&ai_meta.ai_definition).unwrap();
            if !ai_definition.input_should_run(key, subject_entity_id) {
                bevy::prelude::debug!("skipped calculating inputs for this entity");
                continue;
            };
            let targeted_input_filter_sets = res_ai_definitions.map[&ai_meta.ai_definition]
                .targeted_input_filter_sets
                .get(&key);
            let target_entities = match targeted_input_filter_sets {
                Some(target_entity_sets) => Some(
                    target_entity_sets
                        .iter()
                        .map(|&set_key| res_ai_target_entity_sets.get(set_key))
                        .flatten()
                        .collect::<Vec<bevy::prelude::Entity>>(),
                ),
                None => None,
            };
            let score_map = ai_meta
                .targeted_input_scores
                .entry(key)
                .or_insert(bevy::utils::HashMap::new());
            let subject = (p0,);
            if let Some(target_entities) = target_entities {
                bevy::prelude::debug!(
                    "calculating input for {} filter set entities",
                    target_entities.len()
                );
                for &target_entity in &target_entities {
                    let (entity_id, p0) = q_target.get(target_entity).unwrap();
                    let _span =
                        bevy::prelude::debug_span!("", target_entity = entity_id.index())
                            .entered();
                    if entity_id == subject_entity_id {
                        continue;
                    }
                    let target = (p0,);
                    let score = { subject.0.val - target.0.val };
                    let entry = score_map.entry(entity_id).or_insert(f32::NEG_INFINITY);
                    *entry = score;
                    bevy::prelude::debug!("score {:.2}", score);
                }
            } else {
                for (entity_id, p0) in q_target.iter() {
                    let _span =
                        bevy::prelude::debug_span!("", target_entity = entity_id.index())
                            .entered();
                    if entity_id == subject_entity_id {
                        continue;
                    }
                    let target = (p0,);
                    let score = { subject.0.val - target.0.val };
                    let entry = score_map.entry(entity_id).or_insert(f32::NEG_INFINITY);
                    *entry = score;
                    bevy::prelude::debug!("score {:.2}", score);
                }
            }
        }
    }
}
