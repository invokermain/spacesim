use bevy::ecs::query::WorldQuery;
use bevy::{
    prelude::Component,
    prelude::{Bundle, Query},
    reflect::Reflect,
};
use bevy_utility_ai_macros::input_system;
use std::collections::HashMap;

// USER DEFINED STUFF
#[input_system]
fn utility_input_low(some_data: &SomeData) -> f32 {
    some_data.val
}

#[input_system]
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
pub trait UtilityInput {
    fn input_id() -> String;
}

// pub trait AISystem {
//     fn update_input_score();
// }

#[derive(Component)]
pub struct AI {}

#[derive(Component, Default, Clone)]
pub struct AIMeta {
    input_scores: HashMap<usize, f32>,
}

impl AIMeta {
    pub fn new() -> Self {
        Self {
            input_scores: { Default::default() },
        }
    }
}

// creates and configures an AI entity
struct CreateAIBundle<T: Component> {
    ai_meta: AIMeta,
    component: T,
}

impl<T: Component> CreateAIBundle<T> {
    pub fn new(component: T) -> CreateAIBundle<T> {
        Self {
            ai_meta: AIMeta::default(),
            component,
        }
    }

    pub fn add_input<Q: WorldQuery>(mut self, some: fn(Query<Q>)) -> CreateAIBundle<T> {
        self.ai_meta.input_scores.insert(some as usize, -1.0);
        self
    }

    pub fn build(self) -> AIBundle<T> {
        AIBundle {
            ai_meta: self.ai_meta.clone(),
            marker_component: self.component,
        }
    }
}

#[derive(Bundle)]
struct AIBundle<T: Component> {
    ai_meta: AIMeta,
    marker_component: T,
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::*;

    #[test]
    fn test() {
        let mut app = App::new();

        let bundle = CreateAIBundle::new(AI {})
            .add_input(utility_input_system)
            .build();

        app.add_system(utility_input_system);

        let entity_id = app.world.spawn((bundle, SomeData { val: 0.25 })).id();

        app.update();

        let ai_meta = app.world.get::<AIMeta>(entity_id).unwrap();

        assert_eq!(ai_meta.input_scores[&(utility_input_system as usize)], 0.25);
    }
}
