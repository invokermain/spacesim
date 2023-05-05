use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use bevy::{
    ecs::{
        component::{ComponentId, ComponentStorage, TableStorage},
        storage::{SparseSet, Table},
        system::EntityCommands,
        world::{EntityMut, EntityRef},
    },
    prelude::{AppTypeRegistry, Commands, Component, Entity, ReflectComponent, Res, World},
    reflect::{Reflect, TypeRegistry, TypeRegistryInternal},
};

// A Decision Maker is a collections of decisions an Entity can make.
#[derive(Component)]
pub struct DecisionMaker {
    decisions: Vec<Decision>,
    current_decision_score: f32,
    current_action: Option<Box<dyn Action>>,
}

// A Decision Maker is a collections of decisions an Entity can make.
pub struct Decision {
    considerations: Vec<Consideration>,
    action: Arc<dyn Action>,
}

pub struct Consideration {
    input: fn(&EntityRef) -> f32,
}

pub trait Action: Component<Storage = TableStorage> + Reflect {
    fn build(&self, entity: &mut EntityMut);
}

fn decision_maker_system(world: &mut World) {
    let mut keep_action: Vec<(Entity, f32)> = vec![];
    let mut change_action: Vec<(Entity, f32, Arc<dyn Action>)> = vec![];

    {
        let mut q_decision_makers = world.query::<(Entity, &DecisionMaker)>();
        for (entity, decision_maker) in q_decision_makers.iter(&world) {
            let mut decisions = Vec::new();

            for decision in &decision_maker.decisions {
                let mut decision_score = 1.0;

                for consideration in &decision.considerations {
                    let entity = world.get_entity(entity).unwrap();
                    let consideration_score = (consideration.input)(&entity);
                    decision_score *= consideration_score;
                }

                decisions.push((decision, decision_score));
            }

            decisions.sort_by(|a, b| b.1.total_cmp(&a.1));
            let (decision, score) = decisions.first().unwrap();

            if decision.action.as_ref().type_id() == decision_maker.current_action.type_id() {
                keep_action.push((entity, *score));
            } else {
                change_action.push((entity, *score, decision.action.clone()))
            }
        }
    }

    for (entity_id, score) in keep_action {
        let mut decision_maker = world.get_mut::<DecisionMaker>(entity_id).unwrap();
        decision_maker.current_decision_score = score;
    }

    for (entity_id, score, action) in change_action {
        let entity_mut = &mut world.get_entity_mut(entity_id).unwrap();
        action.build(entity_mut);

        let app_type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
        let type_registry = app_type_registry.read();
        let reflection = type_registry
            .get(action.type_id())
            .unwrap()
            .data::<ReflectComponent>()
            .unwrap();

        reflection.remove(entity_mut);

        let mut decision_maker = world.get_mut::<DecisionMaker>(entity_id).unwrap();
        decision_maker.current_decision_score = score;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::*;

    #[test]
    fn test_decision_maker_picks_highest_utility() {
        // INPUTS
        fn utility_input_low(entity: &EntityRef) -> f32 {
            0.25
        }
        fn utility_input_high(entity: &EntityRef) -> f32 {
            0.75
        }

        // ACTION COMPONENTS
        #[derive(Component, Reflect)]
        struct ActionOne {}

        impl Action for ActionOne {
            fn build(&self, entity: &mut EntityMut) {
                entity.insert(Self {});
            }
        }

        #[derive(Component, Reflect)]
        struct ActionTwo {}

        impl Action for ActionTwo {
            fn build(&self, entity: &mut EntityMut) {
                entity.insert(Self {});
            }
        }

        let mut app = App::new();
        app.add_system(decision_maker_system);

        let entity_id = app
            .world
            .spawn(DecisionMaker {
                decisions: vec![
                    Decision {
                        considerations: vec![Consideration {
                            input: utility_input_low,
                        }],
                        action: Arc::new(ActionOne {}),
                    },
                    Decision {
                        considerations: vec![Consideration {
                            input: utility_input_high,
                        }],
                        action: Arc::new(ActionTwo {}),
                    },
                ],
                current_decision_score: 0.0,
                current_action: None,
            })
            .id();

        app.update();

        let decision_maker = app.world.get::<DecisionMaker>(entity_id).unwrap();

        assert_eq!(decision_maker.current_decision_score, 0.75);
    }
}
