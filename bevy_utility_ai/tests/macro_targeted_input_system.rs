mod common;

use crate::common::SomeOtherData;
use bevy::app::App;
use bevy::utils::hashbrown::HashSet;
use bevy_utility_ai::{AIDefinition, AIDefinitions, AIMeta};
use bevy_utility_ai_macros::targeted_input_system;
use common::{SomeData, AI};
use std::any::TypeId;

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
fn simple_targeted_input_system_updates_aimeta_inputs() {
    #[targeted_input_system]
    fn simple_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = App::new();

    app.add_system(simple_targeted_input);

    app.init_resource::<AIDefinitions>();

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![simple_targeted_input as usize]),
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
fn targeted_input_system_produces_valid_system() {
    #[targeted_input_system]
    fn targeted_input(subject: (&SomeOtherData,), target: (&SomeData,)) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = App::new();
    app.add_system(targeted_input);
}

#[test]
fn targeted_input_system_updates_aimeta_inputs() {
    #[targeted_input_system]
    fn targeted_input(subject: (&SomeOtherData,), target: (&SomeData,)) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = App::new();
    app.add_system(targeted_input);

    app.init_resource::<AIDefinitions>();

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![targeted_input as usize]),
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
