use bevy_utility_ai_macros::targeted_input_system;

#[targeted_input_system]
fn simple_targeted_input(subject: (&SomeData,)) -> f32 {
    some_data.val
}

fn main() {}
