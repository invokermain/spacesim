use crate::considerations::Consideration;
use bevy::ecs::component::ComponentDescriptor;
use bevy::prelude::Component;
use bevy::reflect::{GetTypeRegistration, TypeRegistration};
use std::any::{type_name, TypeId};

pub struct Decision {
    pub(crate) name: String,
    pub(crate) action_name: String,
    pub(crate) action: TypeId,
    pub(crate) type_registration: TypeRegistration,
    pub(crate) is_targeted: bool,
    pub(crate) considerations: Vec<Consideration>,
    pub(crate) target_filters: Vec<ComponentDescriptor>,
}

impl Decision {
    pub fn simple<C: Component + GetTypeRegistration>(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted: false,
            considerations: Vec::new(),
            target_filters: Vec::new(),
        }
    }

    pub fn targeted<C: Component + GetTypeRegistration>(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted: true,
            considerations: Vec::new(),
            target_filters: Vec::new(),
        }
    }

    pub fn add_consideration(mut self, consideration: Consideration) -> Self {
        self.considerations.push(consideration);
        self
    }

    pub fn add_target_filter<C: Component>(mut self) -> Self {
        if !self.is_targeted {
            panic!("Only targeted Decisions may have target filters")
        }

        self.target_filters.push(ComponentDescriptor::new::<C>());
        self
    }
}
