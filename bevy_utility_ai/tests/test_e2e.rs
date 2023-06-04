use std::any::TypeId;

use bevy::prelude::{Entity, Query, Vec2};
use bevy::{
    log::LogPlugin,
    prelude::{App, Component, IntoSystemConfig, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::ai_meta::AIMeta;
use bevy_utility_ai::{
    systems::{UtililityAISet, UtilityAIPlugin},
    Consideration, DefineAI,
};
use bevy_utility_ai_macros::input_system;

/// This test checks whether the framework correctly chooses the highest scoring decision in the
/// trivial case of two decisions with one consideration each.
#[test]
fn test() {
    // SETUP
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    #[input_system]
    fn utility_input_high(some_other_data: &SomeOtherData) -> f32 {
        some_other_data.val
    }

    // Marker component for our AI system
    #[derive(Component)]
    pub struct AI {}

    #[derive(Component, Reflect, Default)]
    #[reflect(Component, Default)]
    struct ActionOne {}

    #[derive(Component, Reflect, Default)]
    #[reflect(Component, Default)]
    struct ActionTwo {}

    #[derive(Component)]
    struct SomeData {
        val: f32,
    }

    #[derive(Component)]
    struct SomeOtherData {
        val: f32,
    }
    // END SETUP

    let mut app = App::new();

    app.add_plugin(LogPlugin {
        filter: "info,bevy_utility_ai=debug".into(),
        level: bevy::log::Level::DEBUG,
    });
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_decision::<ActionOne>(vec![Consideration::simple(utility_input_low)])
        .add_decision::<ActionTwo>(vec![Consideration::simple(utility_input_high)])
        .register(&mut app);

    app.register_type::<ActionOne>();
    app.register_type::<ActionTwo>();
    app.add_system(utility_input_low.in_set(UtililityAISet::CalculateInputs));
    app.add_system(utility_input_high.in_set(UtililityAISet::CalculateInputs));

    let entity_id = app
        .world
        .spawn((
            SomeData { val: 0.25 },
            SomeOtherData { val: 0.75 },
            AI {},
            AIMeta::new::<AI>(),
        ))
        .id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(ai_meta.current_action_score, 0.75);
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionTwo>()));
}

/// This test checks whether the framework correctly chooses the highest scoring decision in the
/// trivial targeted case of one decision with one targeted consideration.
#[test]
fn test_targeted() {
    // SETUP
    fn target_selector(mut q_subject: Query<&mut AIMeta>, q_targets: Query<(Entity, &Position)>) {
        let key = targeted_utility_input as usize;
        for mut ai_meta in q_subject.iter_mut() {
            let targets = q_targets.iter().map(|x| x.0).collect();

            if let Some(val) = ai_meta.targeted_input_targets.get_mut(&key) {
                *val = targets;
            } else {
                ai_meta.targeted_input_targets.insert(key, targets);
            }
        }
    }

    fn targeted_utility_input(
        mut q_subject: Query<(Entity, &mut AIMeta, &Position)>,
        q_target: Query<(&Position,)>,
    ) {
        let key = targeted_utility_input as usize;
        for (subject_entity_id, mut ai_meta, position) in q_subject.iter_mut() {
            let subject = (position,);
            let targets = ai_meta.targeted_input_targets[&key].clone();
            for entity_id in targets {
                if entity_id == subject_entity_id {
                    continue;
                }
                let target = q_target.get(entity_id).unwrap();
                let score = subject.0.val.distance(target.0.val).recip();
                let entry = ai_meta
                    .targeted_input_scores
                    .entry((key, entity_id))
                    .or_insert(f32::NEG_INFINITY);
                *entry = score;
            }
        }
    }

    // Marker component for our AI system
    #[derive(Component)]
    pub struct AI {}

    // Marker component for our targets
    #[derive(Component)]
    pub struct Target {}

    #[derive(Component, Reflect, Default)]
    #[reflect(Component, Default)]
    struct ActionOne {}

    #[derive(Component)]
    struct Position {
        val: Vec2,
    }
    // END SETUP

    let mut app = App::new();

    app.add_plugin(LogPlugin {
        filter: "info,bevy_utility_ai=debug".into(),
        level: bevy::log::Level::DEBUG,
    });
    app.add_plugin(UtilityAIPlugin);

    DefineAI::<AI>::new()
        .add_targeted_decision::<ActionOne, (Entity, &Position)>(
            target_selector,
            vec![],
            vec![Consideration::targeted(targeted_utility_input)
                .set_input_name("targeted_utility_input".into())],
        )
        .register(&mut app);

    app.register_type::<ActionOne>();

    app.add_system(target_selector.in_set(UtililityAISet::SelectTargets));
    app.add_system(targeted_utility_input.in_set(UtililityAISet::CalculateInputs));

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
            (
                Target {},
                Position {
                    val: Vec2::new(0., 0.),
                },
            ),
            (
                Target {},
                Position {
                    val: Vec2::new(1., 1.),
                },
            ),
        ])
        .collect::<Vec<Entity>>();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(
        ai_meta.current_action_score,
        Vec2::new(1., 1.).distance(Vec2::new(0.9, 0.9)).recip()
    );
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionOne>()));
    assert_eq!(ai_meta.current_target, Some(target_entitites[1]));
}
