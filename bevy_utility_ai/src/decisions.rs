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
    pub(crate) base_score: f32,
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
            base_score: 1.0,
            considerations: Vec::new(),
            subject_filters: Vec::new(),
            target_filters: Vec::new(),
        }
    }

    pub fn targeted<C: Component + GetTypeRegistration>() -> Self {
        Self {
            action_name: trim_type_name(type_name::<C>()).into(),
            action: TypeId::of::<C>(),
            type_registration: C::get_type_registration(),
            is_targeted: true,
            base_score: 1.0,
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

    /// Set the base score for this decision. The base score is the initial value that gets
    /// multiplied cumatively by each consideration. The default base score is 1.0.
    /// This can be used to either create a fallback decision with no considerations, so that the AI
    /// does something appropriate when there is no good decision to make.
    /// This can also be used to weight decisions at the decision level.
    pub fn set_base_score(mut self, score: f32) -> Self {
        if score <= 0.0 || score >= 10.0 {
            panic!("base_score must be between 0.0 and 10.0");
        }
        self.base_score = score;
        self
    }
}
