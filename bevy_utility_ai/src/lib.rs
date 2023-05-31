pub mod systems;

use bevy::ecs::component::SparseStorage;
use bevy::{
    ecs::query::WorldQuery,
    prelude::{App, Component, Entity, Query, Resource},
    utils::HashMap,
};
use std::any::{type_name, TypeId};
use std::marker::PhantomData;

pub struct AIDefinition {
    decisions: Vec<Decision>,
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    map: HashMap<TypeId, AIDefinition>,
}

/// A Component which stores all the required state to run the AI Systems.
#[derive(Component, Clone)]
pub struct AIMeta {
    ai_definition: TypeId,
    pub input_scores: HashMap<usize, f32>,
    pub targeted_input_scores: HashMap<(usize, Entity), f32>,
    pub targeted_input_targets: HashMap<usize, Vec<Entity>>,
    pub current_action: Option<TypeId>,
    pub current_action_score: f32,
    pub current_action_name: String,
}

impl AIMeta {
    pub fn new<T: Component>() -> Self {
        Self {
            ai_definition: TypeId::of::<T>(),
            input_scores: HashMap::default(),
            targeted_input_scores: HashMap::default(),
            targeted_input_targets: HashMap::default(),
            current_action_score: -1.0,
            current_action: None,
            current_action_name: String::default(),
        }
    }
}

pub struct WithTarget<T: Component> {
    component: T,
    target: Entity,
}

impl<T: Component> Component for WithTarget<T> {
    type Storage = SparseStorage;
}

// Denotes the Target entity ID

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
#[derive(Default)]
pub struct DefineAI<T: Component> {
    input_scores: HashMap<usize, f32>,
    decisions: Vec<Decision>,
    marker_phantom: PhantomData<T>,
}

impl<T: Component> DefineAI<T> {
    pub fn new() -> DefineAI<T> {
        Self {
            marker_phantom: PhantomData,
            decisions: Vec::new(),
            input_scores: HashMap::new(),
        }
    }

    pub fn add_decision<C: Component>(mut self, considerations: Vec<Consideration>) -> DefineAI<T> {
        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            considerations,
            targeted_considerations: vec![],
        };

        // set initial input score
        for consideration in &decision.considerations {
            self.input_scores
                .insert(consideration.input, f32::NEG_INFINITY);
        }

        self.decisions.push(decision);
        self
    }

    pub fn add_targeted_decision<C: Component>(
        mut self,
        considerations: Vec<Consideration>,
        targeted_considerations: Vec<TargetedConsideration>,
    ) -> DefineAI<T> {
        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            considerations,
            targeted_considerations,
        };

        // set initial input score
        for consideration in &decision.considerations {
            self.input_scores
                .insert(consideration.input, f32::NEG_INFINITY);
        }

        self.decisions.push(decision);
        self
    }

    pub fn add_input<Q: WorldQuery>(mut self, input: fn(Query<Q>)) -> DefineAI<T> {
        self.input_scores.insert(input as usize, -1.0);
        self
    }

    pub fn register(self, app: &mut App) {
        app.init_resource::<AIDefinitions>();
        let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();
        ai_definitions.map.insert(
            TypeId::of::<T>(),
            AIDefinition {
                decisions: self.decisions,
            },
        );
    }
}

fn type_name_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(Clone)]
pub struct Consideration {
    pub input_name: String,
    pub input: usize,
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
pub struct TargetedConsideration {
    pub input_name: String,
    pub input: usize,
}

impl TargetedConsideration {
    pub fn new<Q1: WorldQuery, Q2: WorldQuery>(input: fn(Query<Q1>, Query<Q2>)) -> Self {
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
    pub targeted_considerations: Vec<TargetedConsideration>,
}

impl Decision {
    pub fn new<T: Component>(
        considerations: Vec<Consideration>,
        targeted_considerations: Vec<TargetedConsideration>,
    ) -> Self {
        Self {
            action_name: type_name::<T>().into(),
            action: TypeId::of::<T>(),
            considerations,
            targeted_considerations,
        }
    }
}
