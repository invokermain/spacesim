use bevy::prelude::{App, Component, Entity, Query};
use bevy_utility_ai_macros::{input_system, target_selector};

#[derive(Component)]
struct SomeData {
    val: f32,
}

#[test]
fn input_system_macro() {
    #[input_system]
    fn utility_input_low(some_data: &SomeData) -> f32 {
        some_data.val
    }

    // assert it produces a valid bevy system that can run for a single step
    let mut app = App::new();
    app.add_system(utility_input_low);
    app.update();
}

#[test]
fn target_selector_macro() {
    #[target_selector]
    fn my_target_selector(q_targets: Query<Entity>) -> Vec<Entity> {
        q_targets.iter().collect()
    }

    // assert it produces a valid bevy system that can run for a single step
    let mut app = App::new();
    app.add_system(my_target_selector);
    app.update();
}

#[test]
fn target_selector_macro2() {
    #[target_selector]
    fn my_target_selector(q_targets: Query<(Entity, &SomeData)>) -> Vec<Entity> {
        q_targets.iter().map(|x| x.0).collect()
    }

    // assert it produces a valid bevy system that can run for a single step
    let mut app = App::new();
    app.add_system(my_target_selector);
    app.update();
}

// Tests for failing macro builds
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs")
}
