mod common;

use bevy::utils::hashbrown::HashSet;
use bevy::{app::App, utils::HashMap};
use bevy_utility_ai::{AIDefinition, AIDefinitions, AIMeta};
use bevy_utility_ai_macros::input_system;
use common::{SomeData, AI};
use std::any::TypeId;

#[test]
fn input_system_macro_produces_valid_system() {
    // GIVEN
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = App::new();
    app.add_system(utility_input_low);
}

#[test]
fn input_system_macro_updates_aimeta_inputs() {
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = App::new();

    app.add_system(utility_input_low);

    app.init_resource::<AIDefinitions>();

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            required_inputs: HashSet::from_iter(vec![utility_input_low as usize]),
            targeted_input_filter_sets: HashMap::new(),
        },
    );

    let entity_id = app
        .world
        .spawn((SomeData { val: 0.25 }, AI {}, AIMeta::new::<AI>()))
        .id();

    app.update();

    let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

    assert!(ai_meta
        .input_scores
        .contains_key(&(utility_input_low as usize)));
    assert_eq!(ai_meta.input_scores[&(utility_input_low as usize)], 0.25);
}
