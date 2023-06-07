use bevy::prelude::{App, Component};
use bevy_utility_ai_macros::input_system;

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

// Tests for failing macro builds
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs")
}
