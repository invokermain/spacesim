use bevy::prelude::{Entity, EventWriter, Query};

use super::{
    components::{
        CommodityStorage, IsCompany, OnPlanet, OwnedFactories, Population, Production, Wealth,
    },
    events::CommodityProducedEvent,
    market::Market,
};

pub fn company_simulate(
    mut q_company: Query<(
        Entity,
        &IsCompany,
        &mut Wealth,
        &mut CommodityStorage,
        &OnPlanet,
        &OwnedFactories,
    )>,
    q_manufactory: Query<&Production>,
    mut ev_commodity_produced: EventWriter<CommodityProducedEvent>,
) {
    for (entity, _is_company, mut wealth, mut storage, on_planet, owned_factories) in
        q_company.iter_mut()
    {
        if storage.available_capacity > 0.0 {
            // get everything that we can produce
            let all_producable = owned_factories
                .value
                .iter()
                .map(|&entity| q_manufactory.get_component::<Production>(entity).unwrap())
                .collect::<Vec<&Production>>();

            // TODO: calculate expected profit for each commodity
            //       for each commodity get market price - manufacture cost,
            //       then order by expected profit.

            for producable in all_producable {
                let units = f32::min(producable.output_per_tick, storage.available_capacity);
                let cost = units * producable.cost_per_unit;

                // we can produce something
                if wealth.value > cost {
                    storage.store(producable.commodity_type, units);
                    wealth.value -= cost;

                    ev_commodity_produced.send(CommodityProducedEvent {
                        source_entity: entity,
                        commodity_type: producable.commodity_type,
                        change: units,
                    })
                }

                if storage.available_capacity == 0.0 {
                    break;
                }
            }
        }
    }
}

pub fn population_consumption(mut query: Query<(&Population, &Market)>) {
    for (pop, market) in query.iter() {}
}
