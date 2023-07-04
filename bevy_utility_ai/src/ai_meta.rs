use bevy::prelude::{Component, Entity};
use bevy::utils::HashMap;
use std::any::TypeId;

/// A Component which stores all the state required by the various AI systems relating to an Entity.
#[derive(Component, Clone)]
pub struct AIMeta {
    /// The TypeID of the marker component for the AI, can be used to lookup against the
    /// AIDefinitions resource.
    pub ai_definition: TypeId,
    /// A map of the scores for each required non-targeted input for this AI, this is populated
    /// by the relevant input systems.
    pub input_scores: HashMap<TypeId, f32>,
    /// A map of the scores for each required targeted input for this AI, this is populated
    /// by the relevant input systems.
    pub targeted_input_scores: HashMap<TypeId, HashMap<Entity, f32>>,
    /// The TypeId of this entities current action according to the AI.
    pub current_action: Option<TypeId>,
    /// The score of the current action
    pub current_action_score: f32,
    /// The name of the current action
    pub current_action_name: String,
    /// The current target, exists if the current action is a targeted one.
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
