mod common;

use bevy::app::{App, Update};
use bevy::prelude::Res;
use bevy::time::Time;
use bevy::utils::hashbrown::HashSet;
use bevy_utility_ai::utils::type_id_of;
use bevy_utility_ai::{AIDefinition, AIDefinitions, AIMeta};
use bevy_utility_ai_macros::input_system;
use common::{SomeData, AI};
use std::any::TypeId;

#[test]
fn input_system_macro_produces_valid_system() {
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = App::new();
    app.add_systems(Update, utility_input_low);
}

#[test]
fn input_system_macro_with_resource_produces_valid_system() {
    #[input_system]
    fn utility_input_low(some_data: &SomeData, the_time: Res<Time>) -> f32 {
        println!("the time is: {:?}", the_time);
        some_data.val
    }

    let mut app = App::new();
    app.add_systems(Update, utility_input_low);
}

#[test]
fn input_system_macro_updates_aimeta_inputs() {
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    let mut app = App::new();

    app.add_systems(Update, utility_input_low);

    app.init_resource::<AIDefinitions>();

    let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
    ai_definitions.map.insert(
        TypeId::of::<AI>(),
        AIDefinition {
            decisions: vec![], // this field doesn't matter for this test
            simple_inputs: HashSet::from_iter(vec![type_id_of(&utility_input_low)]),
            targeted_inputs: Default::default(),
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
        .contains_key(&type_id_of(&utility_input_low)));
    assert_eq!(ai_meta.input_scores[&type_id_of(&utility_input_low)], 0.25);
}
