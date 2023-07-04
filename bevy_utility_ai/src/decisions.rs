use crate::considerations::Consideration;
use crate::utils::trim_type_name;
use bevy::prelude::Component;
use bevy::reflect::{GetTypeRegistration, TypeRegistration};
use std::any::{type_name, TypeId};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Filter {
    /// Entiy must contain Component with this TypeId
    Inclusive(TypeId),
    /// Entity must not contain Component with this TypeId
    Exclusive(TypeId),
}

impl Filter {
    pub fn component_type_id(&self) -> TypeId {
        match self {
            Filter::Inclusive(t) => *t,
            Filter::Exclusive(t) => *t,
        }
    }
}

pub struct Decision {
    pub(crate) action_name: String,
    pub(crate) action: TypeId,
    pub(crate) type_registration: TypeRegistration,
    pub(crate) is_targeted: bool,
    pub(crate) considerations: Vec<Consideration>,
    pub(crate) subject_filters: Vec<Filter>,
    pub(crate) target_filters: Vec<Filter>,
}

impl Decision {
    pub fn simple<C: Component + GetTypeRegistration>() -> Self {
        Self {
            action_name: trim_type_name(type_name::<C>()).into(),
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted: false,
            considerations: Vec::new(),
            subject_filters: Vec::new(),
            target_filters: Vec::new(),
        }
    }

    pub fn targeted<C: Component + GetTypeRegistration>() -> Self {
        Self {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted: true,
            considerations: Vec::new(),
            subject_filters: Vec::new(),
            target_filters: Vec::new(),
        }
    }

    pub fn add_consideration(mut self, consideration: Consideration) -> Self {
        self.considerations.push(consideration);
        self
    }

    pub fn subject_filter_include<C: Component>(mut self) -> Self {
        self.subject_filters
            .push(Filter::Inclusive(TypeId::of::<C>()));
        self
    }

    pub fn subject_filter_exclude<C: Component>(mut self) -> Self {
        self.subject_filters
            .push(Filter::Exclusive(TypeId::of::<C>()));
        self
    }

    pub fn target_filter_include<C: Component>(mut self) -> Self {
        if !self.is_targeted {
            panic!("Only targeted Decisions may have target filters")
        }

        self.target_filters
            .push(Filter::Inclusive(TypeId::of::<C>()));
        self
    }

    pub fn target_filter_exclude<C: Component>(mut self) -> Self {
        if !self.is_targeted {
            panic!("Only targeted Decisions may have target filters")
        }

        self.target_filters
            .push(Filter::Exclusive(TypeId::of::<C>()));
        self
    }
}
