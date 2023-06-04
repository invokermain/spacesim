pub mod ai_meta;
pub mod response_curves;
pub mod systems;

pub use crate::ai_meta::AIMeta;
use crate::response_curves::{LinearCurve, ResponseCurve};
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

#[derive(Component)]
pub struct ActionTarget {
    target: Entity,
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

    pub fn add_decision<C: Component>(
        mut self,
        considerations: Vec<Consideration>,
    ) -> DefineAI<T> {
        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            considerations,
            targeted_considerations: vec![],
            is_targeted: false,
            target_selector: None,
        };

        // set initial input score
        for consideration in &decision.considerations {
            self.input_scores
                .insert(consideration.input, f32::NEG_INFINITY);
        }

        self.decisions.push(decision);
        self
    }

    pub fn add_targeted_decision<C: Component, Q: WorldQuery>(
        mut self,
        target_selector: fn(Query<&mut AIMeta>, Query<Q>),
        considerations: Vec<Consideration>,
        targeted_considerations: Vec<Consideration>,
    ) -> DefineAI<T> {
        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            considerations,
            targeted_considerations,
            is_targeted: true,
            target_selector: Some(target_selector as usize),
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
    type_name::<T>()
}

pub struct Consideration {
    pub input_name: String,
    pub input: usize,
    pub response_curve: Box<dyn ResponseCurve>,
}

impl Consideration {
    pub fn simple<Q: WorldQuery>(input: fn(Query<Q>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
        }
    }

    pub fn targeted<Q1: WorldQuery, Q2: WorldQuery>(input: fn(Query<Q1>, Query<Q2>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
        }
    }

    pub fn with_response_curve(self, response_curve: impl ResponseCurve + 'static) -> Self {
        Self {
            response_curve: Box::new(response_curve),
            ..self
        }
    }

    pub fn set_input_name(self, input_name: String) -> Self {
        Self { input_name, ..self }
    }
}

pub struct Decision {
    pub action_name: String,
    pub action: TypeId,
    pub considerations: Vec<Consideration>,
    pub targeted_considerations: Vec<Consideration>,
    pub is_targeted: bool,
    pub target_selector: Option<usize>,
}
