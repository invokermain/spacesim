use crate::response_curves::{LinearCurve, ResponseCurve};
use crate::systems::inclusive_targeted_filter_input;
use crate::{AIDefinitions, AITargetEntitySets};
use bevy::app::{IntoSystemAppConfig, SystemAppConfig};
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
    // This is Option to allow it to be Taken out later on as SystemAppConfig does not implement
    // clone.
    pub(crate) system_app_config: Option<SystemAppConfig>,
}

impl Consideration {
    fn construct(
        input_name: String,
        input: usize,
        consideration_type: ConsiderationType,
        system_app_config: SystemAppConfig,
    ) -> Self {
        Self {
            input_name,
            input,
            consideration_type,
            system_app_config: Some(system_app_config),
            response_curve: Box::new(LinearCurve::new(1.0)),
        }
    }

    pub fn simple<Q: WorldQuery + 'static>(input: fn(Query<Q>, Res<AIDefinitions>)) -> Self {
        Consideration::construct(
            type_name_of(input).into(),
            input as usize,
            ConsiderationType::Simple,
            input.into_app_config(),
        )
    }

    pub fn targeted<Q1: WorldQuery + 'static, Q2: WorldQuery + 'static>(
        input: fn(Query<Q1>, Query<Q2>, Res<AIDefinitions>, Res<AITargetEntitySets>),
    ) -> Self {
        Consideration::construct(
            type_name_of(input).into(),
            input as usize,
            ConsiderationType::Targeted,
            input.into_app_config(),
        )
    }

    pub fn targeted_filter<F: Component>() -> Self {
        let input = inclusive_targeted_filter_input::<F>;
        Consideration::construct(
            format!("targeted_filter_{}", type_name::<F>()),
            input as usize,
            ConsiderationType::TargetedFilter,
            input.into_app_config(),
        )
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

    pub fn set_input_name(self, input_name: impl Into<String>) -> Self {
        Self {
            input_name: input_name.into(),
            ..self
        }
    }
}
