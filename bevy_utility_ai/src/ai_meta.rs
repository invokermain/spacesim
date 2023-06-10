use bevy::prelude::{Component, Entity};
use bevy::utils::HashMap;
use std::any::TypeId;

/// A Component which stores all the required state to run the AI Systems.
#[derive(Component, Clone)]
pub struct AIMeta {
    pub ai_definition: TypeId,
    pub input_scores: HashMap<usize, f32>,
    pub targeted_input_scores: HashMap<usize, HashMap<Entity, f32>>,
    pub current_action: Option<TypeId>,
    pub current_action_score: f32,
    pub current_action_name: String,
    pub current_target: Option<Entity>,
}

impl AIMeta {
    pub fn new<T: Component>() -> Self {
        Self {
            ai_definition: TypeId::of::<T>(),
            input_scores: HashMap::default(),
            targeted_input_scores: HashMap::default(),
            current_action_score: -1.0,
            current_action: None,
            current_action_name: String::default(),
            current_target: None,
        }
    }
}
