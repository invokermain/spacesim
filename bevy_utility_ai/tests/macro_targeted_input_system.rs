mod common;

use crate::common::SomeOtherData;
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
