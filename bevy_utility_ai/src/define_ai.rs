use crate::considerations::{Consideration, ConsiderationType};
use crate::systems::ensure_entity_has_ai_meta;
use crate::{AIDefinition, AIDefinitions, Decision};
use bevy::app::{App, AppTypeRegistry};
use bevy::prelude::Component;
use bevy::reflect::{GetTypeRegistration, TypeRegistration};
use bevy::utils::{HashMap, HashSet};
use std::any::{type_name, TypeId};
use std::marker::PhantomData;

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
#[derive(Default)]
pub struct DefineAI<T: Component> {
    decisions: Vec<Decision>,
    required_inputs: HashSet<usize>,
    action_type_registrations: Vec<TypeRegistration>,
    /// A map of targeted_input system to the filter sets required for it
    targeted_input_filter_sets: HashMap<usize, Vec<usize>>,
    marker_phantom: PhantomData<T>,
}

impl<T: Component> DefineAI<T> {
    pub fn new() -> DefineAI<T> {
        Self {
            marker_phantom: PhantomData,
            decisions: Vec::new(),
            required_inputs: HashSet::new(),
            action_type_registrations: Vec::new(),
            targeted_input_filter_sets: HashMap::new(),
        }
    }

    pub fn add_decision<C: Component + GetTypeRegistration>(
        mut self,
        considerations: Vec<Consideration>,
        // target_filter_here
    ) -> DefineAI<T> {
        let mut simple_considerations = Vec::new();
        let mut targeted_filter_considerations = Vec::new();
        let mut targeted_considerations = Vec::new();

        considerations.into_iter().for_each(|consideration| {
            self.required_inputs.insert(consideration.input);
            match consideration.consideration_type {
                ConsiderationType::Simple => simple_considerations.push(consideration),
                ConsiderationType::Targeted => targeted_considerations.push(consideration),
                ConsiderationType::TargetedFilter => {
                    targeted_filter_considerations.push(consideration)
                }
            }
        });

        if !targeted_filter_considerations.is_empty() && targeted_considerations.is_empty() {
            panic!(
                "Decisions that have Consideration::targeted_filter considerations without \
                any Consideration::targeted considerations are invalid!"
            )
        }

        let is_targeted = !targeted_considerations.is_empty();

        // Add any filter considerations to the AIDefinition
        if is_targeted && !targeted_filter_considerations.is_empty() {
            let filter_sets: Vec<usize> = targeted_filter_considerations
                .iter()
                .map(|f| f.input)
                .collect();
            for targeted_consideration in &targeted_considerations {
                self.targeted_input_filter_sets
                    .insert(targeted_consideration.input, filter_sets.clone());
            }
        }

        let decision = Decision {
            action_name: type_name::<C>().into(),
            action: TypeId::of::<C>(),
            simple_considerations,
            targeted_considerations,
            is_targeted,
        };

        self.action_type_registrations
            .push(C::get_type_registration());
        self.decisions.push(decision);
        self
    }

    pub fn register(self, app: &mut App) {
        // note all these actions are idempotent except app.add_system

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
                    required_inputs: self.required_inputs,
                    targeted_input_filter_sets: self.targeted_input_filter_sets,
                },
            );
        } else {
            panic!("AI is already defined for this marker component!")
        }

        // Register actions with the AppTypeRegistry
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut registry_write = registry.write();
            self.action_type_registrations
                .into_iter()
                .for_each(|f| registry_write.add_registration(f));
        }

        // Add utility systems
        app.add_system(ensure_entity_has_ai_meta::<T>);
    }
}
