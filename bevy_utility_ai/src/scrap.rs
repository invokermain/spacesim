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

pub trait Action: Component<Storage = TableStorage> + Reflect {
    fn build(&self, entity: &mut EntityMut);
}
