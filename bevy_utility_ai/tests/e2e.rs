use std::any::TypeId;

use bevy::prelude::{Entity, Vec2};

use bevy_utility_ai::ai_meta::AIMeta;
use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::plugin::UtilityAIPlugin;
use bevy_utility_ai::response_curves::LinearCurve;
use bevy_utility_ai::{input_system, targeted_input_system, AITargetEntitySets};

use crate::common::app::test_app;
use crate::common::{
    ActionOne, ActionTwo, Position, SomeData, SomeOtherData, AA, AI, AI1, AI2,
};

mod common;

/// Test that adding the plugin without any configuration doesn't break the app.
#[test]
fn test_empty_plugin() {
    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);
    app.update();
}

/// This test checks whether the framework correctly chooses the highest scoring decision in the
/// trivial case of two decisions with one consideration each.
#[test]
fn simple_considerations_trivial() {
    // SETUP
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    #[input_system]
    fn utility_input_high(some_other_data: &SomeOtherData) -> f32 {
        some_other_data.val
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_decision::<ActionOne>(vec![Consideration::simple(utility_input_low)
            .set_input_name("utility_input_low".into())])
        .add_decision::<ActionTwo>(vec![Consideration::simple(utility_input_high)
            .set_input_name("utility_input_high".into())])
        .register(&mut app);

    let entity_id = app
        .world
        .spawn((
            SomeData { val: 0.25 },
            SomeOtherData { val: 0.75 },
            AI {},
            AIMeta::new::<AI>(),
        ))
        .id();

    // Double update so that calculate inputs & make decisions runs
    app.update();
    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(ai_meta.current_action_score, 0.75);
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionTwo>()));
}

/// This test checks that the framework does not calculate inputs for entities that
/// do not require it
#[test]
fn calculate_inputs_calculates_only_for_required_entities() {
    // SETUP
    #[input_system]
    fn utility_input_1(some_data: &SomeData) -> f32 {
        some_data.val
    }

    #[input_system]
    fn utility_input_2(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI1>::new()
        .add_decision::<ActionOne>(vec![
            Consideration::simple(utility_input_1).set_input_name("utility_input_1".into())
        ])
        .register(&mut app);

    DefineAI::<AI2>::new()
        .add_decision::<ActionOne>(vec![
            Consideration::simple(utility_input_2).set_input_name("utility_input_2".into())
        ])
        .register(&mut app);

    let entity_1 = app
        .world
        .spawn((SomeData { val: 1.0 }, AI1 {}, AIMeta::new::<AI1>()))
        .id();
    let entity_2 = app
        .world
        .spawn((SomeData { val: 2.0 }, AI2 {}, AIMeta::new::<AI2>()))
        .id();

    // Double update so that calculate inputs & make decisions runs
    app.update();
    app.update();

    let ai_meta_1 = app.world.get::<AIMeta>(entity_1).unwrap();
    let ai_meta_2 = app.world.get::<AIMeta>(entity_2).unwrap();

    assert!(ai_meta_1
        .input_scores
        .contains_key(&(utility_input_1 as usize)));
    assert!(!ai_meta_1
        .input_scores
        .contains_key(&(utility_input_2 as usize)));

    assert!(!ai_meta_2
        .input_scores
        .contains_key(&(utility_input_1 as usize)));
    assert!(ai_meta_2
        .input_scores
        .contains_key(&(utility_input_2 as usize)));
}

/// This test checks whether the framework correctly chooses the highest scoring decision in the
/// trivial targeted case of one decision with one targeted consideration.
#[test]
fn targeted_trivial() {
    // SETUP
    #[targeted_input_system]
    fn targeted_utility_input(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_decision::<ActionOne>(vec![Consideration::targeted(targeted_utility_input)
            .with_response_curve(LinearCurve::new(-1.0).shifted(0.0, 1.0))
            .set_input_name("targeted_utility_input".into())])
        .register(&mut app);

    let entity_id = app
        .world
        .spawn((
            AI {},
            AIMeta::new::<AI>(),
            Position {
                val: Vec2::new(0.9, 0.9),
            },
        ))
        .id();

    let target_entitites = app
        .world
        .spawn_batch(vec![
            (Position {
                val: Vec2::new(0., 0.),
            },),
            (Position {
                val: Vec2::new(1., 1.),
            },),
        ])
        .collect::<Vec<Entity>>();

    // Double update so that calculate inputs & make decisions runs
    app.update();
    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    // assert that we are targeting the closest target
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionOne>()));
    assert_eq!(ai_meta.current_target, Some(target_entitites[1]));
}

/// This test checks that the framework does not calculate targeted inputs for entities that
/// do not require it
#[test]
fn calculate_targeted_inputs_calculates_only_for_required_entities() {
    // SETUP
    #[targeted_input_system]
    fn targeted_utility_input_1(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }

    #[targeted_input_system]
    fn targeted_utility_input_2(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI1>::new()
        .add_decision::<ActionOne>(vec![Consideration::targeted(targeted_utility_input_1)
            .set_input_name("targeted_utility_input_1".into())])
        .register(&mut app);

    DefineAI::<AI2>::new()
        .add_decision::<ActionOne>(vec![Consideration::targeted(targeted_utility_input_2)
            .set_input_name("targeted_utility_input_2".into())])
        .register(&mut app);

    let entity_1 = app
        .world
        .spawn((
            Position {
                val: Vec2::new(1.0, 1.0),
            },
            AI1 {},
            AIMeta::new::<AI1>(),
        ))
        .id();
    let entity_2 = app
        .world
        .spawn((
            Position {
                val: Vec2::new(0.0, 0.0),
            },
            AI2 {},
            AIMeta::new::<AI2>(),
        ))
        .id();

    app.update();

    let ai_meta_1 = app.world.get::<AIMeta>(entity_1).unwrap();
    let ai_meta_2 = app.world.get::<AIMeta>(entity_2).unwrap();

    assert!(ai_meta_1
        .targeted_input_scores
        .contains_key(&(targeted_utility_input_1 as usize)));
    assert!(!ai_meta_1
        .targeted_input_scores
        .contains_key(&(targeted_utility_input_2 as usize)));

    assert!(!ai_meta_2
        .targeted_input_scores
        .contains_key(&(targeted_utility_input_1 as usize)));
    assert!(ai_meta_2
        .targeted_input_scores
        .contains_key(&(targeted_utility_input_2 as usize)));
}

/// This test checks that the framework correctly handles targeted_filter systems.
#[test]
fn calculate_targeted_inputs_respects_filters() {
    // SETUP
    #[targeted_input_system]
    fn targeted_utility_input_1(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI1>::new()
        .add_decision::<ActionOne>(vec![
            Consideration::targeted_filter::<AA>(),
            Consideration::targeted(targeted_utility_input_1)
                .set_input_name("targeted_utility_input_1".into()),
        ])
        .register(&mut app);

    let entity_subject = app
        .world
        .spawn((
            Position {
                val: Vec2::new(1.0, 1.0),
            },
            AI1 {},
            AIMeta::new::<AI1>(),
        ))
        .id();
    let entity_target = app
        .world
        .spawn((
            Position {
                val: Vec2::new(0.0, 0.0),
            },
            AA {},
        ))
        .id();
    let _entity_ignore = app
        .world
        .spawn((Position {
            val: Vec2::new(-1.0, -1.0),
        },))
        .id();

    app.update();

    // Assert that the only score calculated is for entity_target
    let ai_meta = app.world.get::<AIMeta>(entity_subject).unwrap();

    let scores = ai_meta
        .targeted_input_scores
        .get(&(targeted_utility_input_1 as usize))
        .unwrap();

    assert!(scores.contains_key(&entity_target));
    assert_eq!(scores.len(), 1);

    // Assert that the AITargetEntitySets contains only the entity_target
    let ai_target_entity_sets = app.world.get_resource::<AITargetEntitySets>().unwrap();

    let entity_set = ai_target_entity_sets.get(Consideration::targeted_filter::<AA>().input);

    assert_eq!(entity_set.len(), 1);
    assert!(entity_set.contains(&entity_target));
}
