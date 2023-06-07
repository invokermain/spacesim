pub mod ai_meta;
pub mod response_curves;
pub mod systems;
pub use bevy_utility_ai_macros::{input_system, targeted_input_system};

pub use crate::ai_meta::AIMeta;
use crate::response_curves::{LinearCurve, ResponseCurve};
use crate::systems::ensure_entity_has_ai_meta;
use bevy::{
    ecs::query::WorldQuery,
    prelude::{App, Component, Entity, Query, Resource},
    utils::HashMap,
};
use std::any::{type_name, TypeId};
use std::marker::PhantomData;

pub struct AIDefinition {
    pub decisions: Vec<Decision>,
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    pub map: HashMap<TypeId, AIDefinition>,
}

#[derive(Component)]
pub struct ActionTarget {
    pub target: Entity,
}

// Denotes the Target entity ID

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
#[derive(Default)]
pub struct DefineAI<T: Component> {
    decisions: Vec<Decision>,
    marker_phantom: PhantomData<T>,
}

impl<T: Component> DefineAI<T> {
    pub fn new() -> DefineAI<T> {
        Self {
            marker_phantom: PhantomData,
            decisions: Vec::new(),
        }
    }

    pub fn add_decision<C: Component>(
        mut self,
        considerations: Vec<Consideration>,
    ) -> DefineAI<T> {
        let mut simple_considerations = Vec::new();
        let mut targeted_considerations = Vec::new();

        considerations
            .into_iter()
            .for_each(|consideration| match consideration.is_targeted {
                true => {
                    targeted_considerations.push(consideration);
                }
                false => {
                    simple_considerations.push(consideration);
                }
            });

        let is_targeted = !targeted_considerations.is_empty();

        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            simple_considerations,
            targeted_considerations,
            is_targeted,
        };

        self.decisions.push(decision);
        self
    }

    pub fn register(self, app: &mut App) {
        app.init_resource::<AIDefinitions>();
        app.add_system(ensure_entity_has_ai_meta::<T>);
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
    pub is_targeted: bool,
}

impl Consideration {
    pub fn simple<Q: WorldQuery>(input: fn(Query<Q>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
            is_targeted: false,
        }
    }

    pub fn targeted<Q1: WorldQuery, Q2: WorldQuery>(input: fn(Query<Q1>, Query<Q2>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
            is_targeted: true,
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
    pub simple_considerations: Vec<Consideration>,
    pub targeted_considerations: Vec<Consideration>,
    pub is_targeted: bool,
}
