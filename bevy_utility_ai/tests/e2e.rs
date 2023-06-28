use std::any::TypeId;

use bevy::prelude::{Entity, Vec2};

use bevy_utility_ai::ai_meta::AIMeta;
use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::plugin::UtilityAIPlugin;
use bevy_utility_ai::response_curves::LinearCurve;
use bevy_utility_ai::{input_system, targeted_input_system};

use crate::common::app::test_app;
use crate::common::{
    ActionOne, ActionTwo, Position, SomeData, SomeOtherData, AA, AI, AI1, AI2, BB, CC,
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
        .add_decision(Decision::simple::<ActionOne>().add_consideration(
            Consideration::simple(utility_input_low).set_input_name("utility_input_low"),
        ))
        .add_decision(Decision::simple::<ActionTwo>().add_consideration(
            Consideration::simple(utility_input_high).set_input_name("utility_input_high"),
        ))
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
        .add_decision(Decision::simple::<ActionOne>().add_consideration(
            Consideration::simple(utility_input_1).set_input_name("utility_input_1"),
        ))
        .register(&mut app);

    DefineAI::<AI2>::new()
        .add_decision(Decision::simple::<ActionTwo>().add_consideration(
            Consideration::simple(utility_input_2).set_input_name("utility_input_2"),
        ))
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
        .add_decision(
            Decision::targeted::<ActionOne>().add_consideration(
                Consideration::targeted(targeted_utility_input)
                    .with_response_curve(LinearCurve::new(-1.0).shifted(0.0, 1.0))
                    .set_input_name("targeted_utility_input"),
            ),
        )
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

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    // assert that we are targeting the closest target
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionOne>()));
    assert_eq!(ai_meta.current_target, Some(target_entitites[1]));
}

#[test]
fn simple_considerations_respects_subject_filter() {
    // SETUP
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_decision(
            Decision::simple::<ActionOne>()
                .add_consideration(
                    Consideration::simple(utility_input_low)
                        .set_input_name("utility_input_low"),
                )
                .subject_filter_include::<AA>(),
        )
        .register(&mut app);

    let entity_id = app
        .world
        .spawn((SomeData { val: 0.25 }, AI {}, AIMeta::new::<AI>(), AA {}))
        .id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(ai_meta.current_action_score, 0.25);
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionOne>()));
}

#[test]
fn simple_considerations_respects_subject_filter_two() {
    // SETUP
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_decision(
            Decision::simple::<ActionOne>()
                .add_consideration(
                    Consideration::simple(utility_input_low)
                        .set_input_name("utility_input_low"),
                )
                .subject_filter_include::<AA>(),
        )
        .register(&mut app);

    let entity_id = app
        .world
        .spawn((SomeData { val: 0.25 }, AI {}, AIMeta::new::<AI>()))
        .id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(ai_meta.current_action_score, -1.0);
    assert_eq!(ai_meta.current_action, None);
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
        .add_decision(
            Decision::targeted::<ActionOne>().add_consideration(
                Consideration::targeted(targeted_utility_input_1)
                    .set_input_name("targeted_utility_input_1"),
            ),
        )
        .register(&mut app);

    DefineAI::<AI2>::new()
        .add_decision(
            Decision::targeted::<ActionOne>().add_consideration(
                Consideration::targeted(targeted_utility_input_2)
                    .set_input_name("targeted_utility_input_2"),
            ),
        )
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

/// This test checks that the framework correctly handles target filters in the trivial case.
#[test]
fn calculate_targeted_inputs_respects_filters_trivial() {
    // SETUP
    #[targeted_input_system]
    fn targeted_utility_input_1(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }
    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI1>::new()
        .add_decision(
            Decision::targeted::<ActionOne>()
                .add_consideration(
                    Consideration::targeted(targeted_utility_input_1)
                        .set_input_name("targeted_utility_input_1"),
                )
                .target_filter_include::<AA>(),
        )
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
}

/// This test checks that the framework correctly handles target filters in the complex case.
#[test]
fn calculate_targeted_inputs_respects_filters_complex() {
    // SETUP
    #[targeted_input_system]
    fn targeted_utility_input_1(subject: (&Position,), target: (&Position,)) -> f32 {
        subject.0.val.distance(target.0.val)
    }
    let mut app = test_app();
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI1>::new()
        .add_decision(
            Decision::targeted::<ActionOne>()
                .add_consideration(
                    Consideration::targeted(targeted_utility_input_1)
                        .set_input_name("targeted_utility_input_1"),
                )
                .target_filter_include::<AA>()
                .target_filter_include::<BB>(),
        )
        .add_decision(
            Decision::targeted::<ActionTwo>()
                .add_consideration(
                    Consideration::targeted(targeted_utility_input_1)
                        .set_input_name("targeted_utility_input_1"),
                )
                .target_filter_include::<AA>(),
        )
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
            BB {},
        ))
        .id();
    let entity_target_2 = app
        .world
        .spawn((
            Position {
                val: Vec2::new(-1.0, -1.0),
            },
            AA {},
        ))
        .id();
    let _entity_ignore = app
        .world
        .spawn((
            Position {
                val: Vec2::new(-1.0, -1.0),
            },
            CC {},
        ))
        .id();

    app.update();

    // Assert that the only score calculated is for entity_target
    let ai_meta = app.world.get::<AIMeta>(entity_subject).unwrap();

    let scores = ai_meta
        .targeted_input_scores
        .get(&(targeted_utility_input_1 as usize))
        .unwrap();

    assert_eq!(scores.len(), 2);
    assert!(scores.contains_key(&entity_target));
    assert!(scores.contains_key(&entity_target_2));
}
