use std::any::TypeId;

use bevy::{
    ecs::{component::SparseStorage, query::ReadOnlyWorldQuery},
    prelude::Component,
    prelude::Entity,
    reflect::Reflect,
};

// USER DEFINED STUFF
fn utility_input_low(some_data: &SomeData) -> f32 {
    some_data.val
}
fn utility_input_high(some_other_data: &SomeOtherData) -> f32 {
    some_other_data.val
}
#[derive(Component, Reflect)]
struct ActionOne {}

#[derive(Component, Reflect)]
struct ActionTwo {}

#[derive(Component)]
struct SomeData {
    val: f32,
}

#[derive(Component)]
struct SomeOtherData {
    val: f32,
}

// LIB STUFF

// Utility Input needs to be a generic trait as needs to enforce method? Could it somehow
// be a function instead?
pub trait UtilityInput<Q: ReadOnlyWorldQuery> {
    fn get(&self, query: Q, entity_id: Entity) -> f32;
}

// Consideration is generic and stores metadata along with the input function.
pub struct Consideration<Q: ReadOnlyWorldQuery> {
    utility_input: dyn UtilityInput<Q>,
    // response_curve and other fields
}

// A Decision will have 1..N Consideration structs, so we need a trait that treats them
// all as the same.
pub trait Considerations<Q: ReadOnlyWorldQuery> {
    fn get_score(&self, query: Q, entity_id: Entity) -> f32;
}

pub struct Decision<Q: ReadOnlyWorldQuery, C: Considerations<Q>> {
    query: Q,
    considerations: C,
    action: dyn Component<Storage = SparseStorage>,
}

// An Intelligence system will have 1..N Decision structs, so we need a trait that treats
// them all as the same.
pub trait Intelligence<Q: ReadOnlyWorldQuery>: Component {
    fn evaluate(&self, query: Q, entity_id: Entity) -> (f32, TypeId);
}

fn ai_system<I: Intelligence<Q>>(
    q_entities: Query<(Entity, &ShipAI)>,
    q_a: Query<&SomeData>,
    q_b: Query<&SomeOtherData>,
) {
    for (entity_id, ship_ai) in q_entities.iter() {
        let mut decision_scores = vec![];

        // decision one
        decision_scores.push({
            let mut decision_score = 1.0;
            // consideration one
            let some_data = q_a.get(entity_id).unwrap();
            decision_score *= utility_input_low(some_data);

            (decision_score, TypeId::of::<ActionOne>())
        });

        // decision two
        decision_scores.push({
            let mut decision_score = 1.0;
            // consideration one
            let some_other_data = q_b.get(entity_id).unwrap();
            decision_score *= utility_input_high(some_other_data);

            (decision_score, TypeId::of::<ActionOne>())
        });

        decision_scores.sort_by(|a, b| b.0.total_cmp(&a.0));
        let (score, action) = decision_scores.first().unwrap();

        println!("best score: {:?}", score);

        if Some(*action) != ship_ai.current_action {
            println!("Should set new action: {:?}", action);
        }
    }
}
