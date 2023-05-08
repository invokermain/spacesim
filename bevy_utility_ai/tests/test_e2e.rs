use std::any::TypeId;

use bevy::{
    log::LogPlugin,
    prelude::{App, Component, IntoSystemConfig, ReflectComponent, ReflectDefault},
    reflect::Reflect,
};
use bevy_utility_ai::{
    systems::{UtililityAISet, UtilityAIPlugin},
    AIDefinition, AIMeta, Consideration,
};
use bevy_utility_ai_macros::input_system;

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

    let bundle = AIDefinition::new(AI {})
        .add_decision::<ActionOne>(vec![Consideration::new(utility_input_low)])
        .add_decision::<ActionTwo>(vec![Consideration::new(utility_input_high)])
        .create_bundle();

    app.register_type::<ActionOne>();
    app.register_type::<ActionTwo>();
    app.add_system(utility_input_low.in_set(UtililityAISet::CalculateInputs));
    app.add_system(utility_input_high.in_set(UtililityAISet::CalculateInputs));

    let entity_id = app
        .world
        .spawn((bundle, SomeData { val: 0.25 }, SomeOtherData { val: 0.75 }))
        .id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert_eq!(ai_meta.current_action_score, 0.75);
    assert_eq!(ai_meta.current_action, Some(TypeId::of::<ActionTwo>()));
}
