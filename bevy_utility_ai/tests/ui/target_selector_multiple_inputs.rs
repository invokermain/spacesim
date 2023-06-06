use bevy_utility_ai_macros::target_selector;

#[target_selector]
fn my_target_selector(q_a: Query<A>, q_a: Query<B>) -> Vec<Entity> {
    q_targets.iter().map(|x| x.0).collect();
}

fn main() {}
