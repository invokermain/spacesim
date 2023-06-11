use crate::response_curves::{LinearCurve, ResponseCurve};
use crate::systems::inclusive_filter_input;
use crate::AIDefinitions;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::{Component, Query, Res};
use std::any::type_name;

fn type_name_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConsiderationType {
    Simple,
    Targeted,
    TargetedFilter,
}

pub struct Consideration {
    pub input_name: String,
    pub input: usize,
    pub response_curve: Box<dyn ResponseCurve>,
    pub consideration_type: ConsiderationType,
}

impl Consideration {
    pub fn simple<Q: WorldQuery>(input: fn(Query<Q>, Res<AIDefinitions>)) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
            consideration_type: ConsiderationType::Simple,
        }
    }

    pub fn targeted<Q1: WorldQuery, Q2: WorldQuery>(
        input: fn(Query<Q1>, Query<Q2>, Res<AIDefinitions>),
    ) -> Self {
        Self {
            input_name: type_name_of(input).into(),
            input: input as usize,
            response_curve: Box::new(LinearCurve::new(1.0)),
            consideration_type: ConsiderationType::Targeted,
        }
    }

    pub fn targeted_filter<F: Component>() -> Self {
        let input = inclusive_filter_input::<F> as usize;
        Self {
            input_name: format!("targeted_filter_{}", type_name::<F>()),
            input,
            response_curve: Box::new(LinearCurve::new(1.0)),
            consideration_type: ConsiderationType::TargetedFilter,
        }
    }

    pub fn with_response_curve(self, response_curve: impl ResponseCurve + 'static) -> Self {
        if self.consideration_type == ConsiderationType::TargetedFilter {
            panic!("Changing the response curve of a targeted filter is not supported!")
        }
        Self {
            response_curve: Box::new(response_curve),
            ..self
        }
    }

    pub fn set_input_name(self, input_name: String) -> Self {
        Self { input_name, ..self }
    }
}
