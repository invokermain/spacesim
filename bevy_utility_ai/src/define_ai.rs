use crate::considerations::ConsiderationType;
use crate::decisions::Decision;
use crate::plugin::UtililityAISet;
use crate::systems::ensure_entity_has_ai_meta;
use crate::{AIDefinition, AIDefinitions, FilterDefinition, TargetedInputRequirements};
use bevy::{
    app::{App, AppTypeRegistry},
    prelude::Component,
    prelude::IntoSystemConfig,
    prelude::Resource,
    reflect::TypeRegistration,
    utils::{HashMap, HashSet},
};
use std::any::TypeId;
use std::marker::PhantomData;

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
#[derive(Default)]
pub struct DefineAI<T: Component> {
    /// The decisions that make up this AI's logic, passed to AIDefinition on register.
    decisions: Vec<Decision>,
    /// The simple inputs used for this AI, passed to AIDefinition on register.
    simple_inputs: HashSet<usize>,
    /// The targeted inputs used for this AI, passed to AIDefinition on register.
    targeted_inputs: HashMap<usize, TargetedInputRequirements>,
    /// A vec of all actions defined as part of this AI, will be registered to the App.
    action_type_registrations: Vec<TypeRegistration>,
    marker_phantom: PhantomData<T>,
}

impl<T: Component> DefineAI<T> {
    pub fn new() -> DefineAI<T> {
        Self {
            marker_phantom: PhantomData,
            decisions: Vec::new(),
            simple_inputs: HashSet::new(),
            targeted_inputs: HashMap::new(),
            action_type_registrations: Vec::new(),
        }
    }

    pub fn add_decision(mut self, decision: Decision) -> DefineAI<T> {
        for consideration in &decision.considerations {
            match consideration.consideration_type {
                ConsiderationType::Simple => {
                    self.simple_inputs.insert(consideration.input);
                }
                ConsiderationType::Targeted => {
                    let filter_definition = match &decision.target_filters.is_empty() {
                        true => FilterDefinition::Any,
                        false => {
                            FilterDefinition::Filtered(vec![decision.target_filters.clone()])
                        }
                    };
                    if let Some(req) = self.targeted_inputs.get_mut(&consideration.input) {
                        req.target_filter = req.target_filter.merge(&filter_definition)
                    } else {
                        self.targeted_inputs.insert(
                            consideration.input,
                            TargetedInputRequirements {
                                target_filter: filter_definition,
                            },
                        );
                    }
                }
            };
        }

        self.action_type_registrations
            .push(decision.type_registration.clone());
        self.decisions.push(decision);

        self
    }

    /// Registers the defined AI against the bevy App, this should be called as the last step of
    /// the defineAI process.
    pub fn register(mut self, app: &mut App) {
        // note all these actions are idempotent except app.add_system, so we maintain a resource on
        // the app to track systems that are already added.
        {
            let mut added_systems = app
                .world
                .remove_resource::<AddedSystemTracker>()
                .unwrap_or_else(|| {
                    panic!("Make sure the plugin is added to the app before calls to DefineAI")
                });

            app.add_system(ensure_entity_has_ai_meta::<T>);

            // Add utility systems
            for decision in &mut self.decisions {
                decision.considerations.iter_mut().for_each(|c| {
                    let system_app_config = c.system_app_config.take().unwrap();
                    if !added_systems.systems.contains(&c.input) {
                        app.add_system(
                            system_app_config.in_set(UtililityAISet::CalculateInputs),
                        );
                        added_systems.systems.insert(c.input);
                    }
                });
            }

            app.world.insert_resource(added_systems);
        }

        // Register actions with the AppTypeRegistry
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut registry_write = registry.write();
            self.action_type_registrations
                .into_iter()
                .for_each(|f| registry_write.add_registration(f));
        }

        // Add the AIDefinition to the AIDefinitions resource

        let mut ai_definitions = app
            .world
            .get_resource_mut::<AIDefinitions>()
            .unwrap_or_else(|| {
                panic!("Make sure the plugin is added to the app before calls to DefineAI")
            });
        if !ai_definitions.map.contains_key(&TypeId::of::<T>()) {
            ai_definitions.map.insert(
                TypeId::of::<T>(),
                AIDefinition {
                    decisions: self.decisions,
                    simple_inputs: self.simple_inputs,
                    targeted_inputs: self.targeted_inputs,
                },
            );
        } else {
            panic!("AI is already defined for this marker component!")
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct AddedSystemTracker {
    pub(crate) systems: HashSet<usize>,
}
