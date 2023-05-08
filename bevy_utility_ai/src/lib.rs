pub mod systems;

use bevy::ecs::query::WorldQuery;
use bevy::ecs::system::Command;
use bevy::prelude::{AppTypeRegistry, Commands, World};
use bevy::reflect::{GetTypeRegistration, Reflect};
use bevy::{
    prelude::Component,
    prelude::{Bundle, Query},
};
use std::any::{type_name, TypeId};
use std::collections::HashMap;

pub trait Action: Component + Reflect {}

/// A Component which stores all the required state to run the AI Systems.
#[derive(Component, Default, Clone)]
pub struct AIMeta {
    pub input_scores: HashMap<usize, f32>,
    pub decisions: Vec<Decision>,
    pub current_action: Option<TypeId>,
    pub current_action_score: f32,
    pub current_action_name: String,
}

impl AIMeta {
    pub fn new() -> Self {
        Self {
            current_action_score: -1.0,
            ..Default::default()
        }
    }
}

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
pub struct AIDefinition<T: Component> {
    ai_meta: AIMeta,
    component: T,
}

impl<T: Component> AIDefinition<T> {
    pub fn new(component: T) -> AIDefinition<T> {
        Self {
            ai_meta: AIMeta::default(),
            component,
        }
    }

    pub fn add_decision<C: GetTypeRegistration + Component>(
        mut self,
        considerations: Vec<Consideration>,
    ) -> AIDefinition<T> {
        let decision = Decision {
            action_name: type_name::<T>().into(),
            action: TypeId::of::<T>(),
            considerations,
        };

        // set initial input score
        for consideration in &decision.considerations {
            self.ai_meta
                .input_scores
                .insert(consideration.input, f32::NEG_INFINITY);
        }

        self.ai_meta.decisions.push(decision);
        self
    }

    pub fn add_input<Q: WorldQuery>(mut self, input: fn(Query<Q>)) -> AIDefinition<T> {
        self.ai_meta.input_scores.insert(input as usize, -1.0);
        self
    }

    pub fn register(self, _commands: &mut Commands) -> AIDefinition<T> {
        self
    }

    pub fn create_bundle(self) -> AIBundle<T> {
        AIBundle {
            ai_meta: self.ai_meta.clone(),
            marker_component: self.component,
        }
    }
}

#[derive(Clone)]
pub struct Consideration {
    pub input_name: String,
    pub input: usize,
    // could add a default behaviour at some point, e.g. IGNORE, DISQUALIFY, DEFAULT_VALUE
}

fn type_name_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

impl Consideration {
    pub fn new<Q: WorldQuery>(input: fn(Query<Q>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
        }
    }
}

#[derive(Clone)]
pub struct Decision {
    pub action_name: String,
    pub action: TypeId,
    pub considerations: Vec<Consideration>,
}

impl Decision {
    pub fn new<T: Component>(considerations: Vec<Consideration>) -> Self {
        Self {
            action_name: type_name::<T>().into(),
            action: TypeId::of::<T>(),
            considerations,
        }
    }
}

/// A Bevy bundle that can be used to add AI logic to your entity.
/// These are not intended to be created manually.
#[derive(Bundle)]
pub struct AIBundle<T: Component> {
    ai_meta: AIMeta,
    marker_component: T,
}
