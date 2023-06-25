use bevy::prelude::Component;
use std::borrow::Cow;

#[derive(Component, Clone)]
pub struct Name {
    pub value: Cow<'static, str>,
}

impl Name {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Name { value: name.into() }
    }
}
