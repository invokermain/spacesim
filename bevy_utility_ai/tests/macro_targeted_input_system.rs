mod common;

use crate::common::{SomeOtherData, AA};
use bevy::app::Update;
use bevy::prelude::Res;
use bevy::{app::App, prelude::Time, utils::HashMap};
use bevy_utility_ai::decisions::Filter;
use bevy_utility_ai::utils::type_id_of;
use bevy_utility_ai::{AIDefinition, AIDefinitions, AIMeta};
use bevy_utility_ai::{FilterDefinition, TargetedInputRequirements};
use bevy_utility_ai_macros::targeted_input_system;
use common::{SomeData, AI};
use std::any::TypeId;

fn test_app() -> App {
    let mut app = App::new();
    app.init_resource::<AIDefinitions>();
    app
}

#[test]
fn simple_targeted_input_system_produces_valid_system() {
    #[targeted_input_system]
    fn simple_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = App::new();
    app.add_systems(Update, simple_targeted_input);
}

#[test]
fn simple_targeted_input_system_with_resource_produces_valid_system() {
    #[targeted_input_system]
    fn simple_targeted_input_with_resource(target: (&SomeData,), _r_time: Res<Time>) -> f32 {
        target.0.val
    }

    let mut app = App::new();
    app.add_systems(Update, simple_targeted_input_with_resource);
}

#[test]
fn targeted_input_system_produces_valid_system() {
    #[targeted_input_system]
    fn targeted_input(subject: (&SomeOtherData,), target: (&SomeData,)) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = App::new();
    app.add_systems(Update, targeted_input);
}

#[test]
fn targeted_input_system_with_resource_produces_valid_system() {
    #[targeted_input_system]
    fn targeted_input_with_resource(
        subject: (&SomeOtherData,),
        target: (&SomeData,),
        _r_time: Res<Time>,
    ) -> f32 {
        subject.0.val - target.0.val
    }

    let mut app = App::new();
    app.add_systems(Update, targeted_input_with_resource);
}

#[test]
fn trivial_targeted_input_system_updates_aimeta_inputs() {
    #[targeted_input_system]
    fn trivial_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = test_app();

    app.add_systems(Update, trivial_targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            simple_inputs: Default::default(),
            targeted_inputs: HashMap::from_iter(vec![(
                type_id_of(&trivial_targeted_input),
                TargetedInputRequirements {
                    target_filter: FilterDefinition::Any,
                },
            )]),
        },
    );

    let subject_entity_id = app.world.spawn((AI {}, AIMeta::new::<AI>())).id();
    let target_entity_id = app.world.spawn(SomeData { val: 0.25 }).id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(subject_entity_id).unwrap();

    assert_eq!(ai_meta.targeted_input_scores.len(), 1);
    assert!(ai_meta
        .targeted_input_scores
        .contains_key(&type_id_of(&trivial_targeted_input)));
    assert_eq!(
        ai_meta.targeted_input_scores[&type_id_of(&trivial_targeted_input)][&target_entity_id],
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

    app.add_systems(Update, targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            simple_inputs: Default::default(),
            targeted_inputs: HashMap::from_iter(vec![(
                type_id_of(&targeted_input),
                TargetedInputRequirements {
                    target_filter: FilterDefinition::Any,
                },
            )]),
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
        .contains_key(&type_id_of(&targeted_input)));
    assert_eq!(
        ai_meta.targeted_input_scores[&type_id_of(&targeted_input)][&target_entity_id],
        0.5
    );
}

#[test]
fn trivial_targeted_input_system_respects_filter_set() {
    #[targeted_input_system]
    fn trivial_targeted_input(target: (&SomeData,)) -> f32 {
        target.0.val
    }

    let mut app = test_app();
    app.add_systems(Update, trivial_targeted_input);

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            simple_inputs: Default::default(),
            targeted_inputs: HashMap::from_iter(vec![(
                type_id_of(&trivial_targeted_input),
                TargetedInputRequirements {
                    target_filter: FilterDefinition::Filtered(vec![vec![Filter::Inclusive(
                        TypeId::of::<AA>(),
                    )]]),
                },
            )]),
        },
    );

    // spawn some entities
    let entity_subject = app.world.spawn((AI {}, AIMeta::new::<AI>())).id();
    let entity_ignore = app.world.spawn(SomeData { val: 0.25 }).id();
    let entity_target = app.world.spawn((SomeData { val: 0.75 }, AA {})).id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_subject).unwrap();

    assert_eq!(ai_meta.targeted_input_scores.len(), 1);
    assert!(ai_meta
        .targeted_input_scores
        .contains_key(&type_id_of(&trivial_targeted_input)));
    assert_eq!(
        ai_meta.targeted_input_scores[&type_id_of(&trivial_targeted_input)][&entity_target],
        0.75
    );
    assert!(
        !ai_meta.targeted_input_scores[&type_id_of(&trivial_targeted_input)]
            .contains_key(&entity_ignore)
    );
}
