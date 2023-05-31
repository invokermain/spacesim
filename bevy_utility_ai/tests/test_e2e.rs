use std::any::TypeId;

use bevy::math::IVec2;
use bevy::prelude::{Entity, Query, Vec2};
use bevy::{
    log::LogPlugin,
    prelude::{App, Component, IntoSystemConfig, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::{
    systems::{UtililityAISet, UtilityAIPlugin},
    AIMeta, Consideration, DefineAI, TargetedConsideration,
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
        .add_decision::<ActionOne>(vec![Consideration::new(utility_input_low)])
        .add_decision::<ActionTwo>(vec![Consideration::new(utility_input_high)])
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
    // fn targeted_utility_input(subject: (&Position, ), target: (&Position, )) -> f32 {
    //     subject.0.val.distance(target.0.val)
    // }

    fn targets(q: Query<Entity>) {}

    fn targeted_utility_input(
        mut q_subject: Query<(&mut AIMeta, &Position)>,
        q_target: Query<(&Position,)>,
    ) {
        let key = targeted_utility_input as usize;
        for (mut ai_meta, position) in q_subject.iter_mut() {
            let subject = (position,);
            for &entity_id in &ai_meta.targeted_input_targets[&key] {
                let target = q_target.get(entity_id).unwrap();
                let score = subject.0.val.distance(target.0.val);
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
        .add_targeted_decision::<ActionOne>(
            vec![],
            vec![TargetedConsideration::new(targeted_utility_input)],
        )
        .register(&mut app);

    app.register_type::<ActionOne>();
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

    {
        app.world.spawn_batch(vec![
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
        ]);
    }

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();
}
