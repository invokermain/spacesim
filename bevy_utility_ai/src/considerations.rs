use crate::response_curves::{LinearCurve, ResponseCurve};
use crate::utils;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::IntoSystemConfigs;
use std::any::TypeId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConsiderationType {
    Simple,
    Targeted,
}

pub struct Consideration {
    pub input_name: String,
    pub input: TypeId,
    pub response_curve: Box<dyn ResponseCurve>,
    pub consideration_type: ConsiderationType,
    // This is Option to allow it to be Taken out later on as SystemAppConfig does not implement
    // clone.
    pub(crate) system_app_config: Option<SystemConfigs>,
}

impl Consideration {
    fn construct(
        input_name: String,
        input: TypeId,
        consideration_type: ConsiderationType,
        system_app_config: SystemConfigs,
    ) -> Self {
        Self {
            input_name,
            input,
            consideration_type,
            system_app_config: Some(system_app_config),
            response_curve: Box::new(LinearCurve::new(1.0)),
        }
    }

    pub fn simple<M>(input: impl IntoSystemConfigs<M> + 'static) -> Self {
        Consideration::construct(
            utils::trim_type_name(utils::type_name_of(&input)).into(),
            utils::type_id_of(&input),
            ConsiderationType::Simple,
            input.into_configs(),
        )
    }

    pub fn targeted<M>(input: impl IntoSystemConfigs<M> + 'static) -> Self {
        Consideration::construct(
            utils::trim_type_name(utils::type_name_of(&input)).into(),
            utils::type_id_of(&input),
            ConsiderationType::Targeted,
            input.into_configs(),
        )
    }

    pub fn with_response_curve(self, response_curve: impl ResponseCurve + 'static) -> Self {
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
