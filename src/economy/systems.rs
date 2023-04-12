use bevy::{
    prelude::{Entity, EventWriter, Query},
};

use super::{
    components::{CommodityStorage, Production, Wealth, Population},
    events::CommodityProducedEvent, market::Market,
};

pub fn company_production(
    mut query: Query<(Entity, &mut Wealth, &Production, &mut CommodityStorage)>,
    mut ev_commodity_produced: EventWriter<CommodityProducedEvent>,
) {
    for (entity, mut wealth, production, mut storage) in query.iter_mut() {
        if storage.available_capacity > 0.0 {
            for (commodity_idx, producable) in production.info.iter().enumerate() {
                if let Some(producable) = producable {
                    let units = f32::min(producable.output_per_tick, storage.available_capacity);
                    let cost = units * producable.cost_per_unit;

                    // we can produce something
                    if wealth.value > cost {
                        storage.store(commodity_idx.into(), units);
                        wealth.value -= cost;

                        ev_commodity_produced.send(CommodityProducedEvent {
                            source_entity: entity,
                            commodity_type: commodity_idx.into(),
                            change: units,
                        })
                    }
                }
            }
        }
    }
}

pub fn population_consumption(mut query: Query<(&Population, &Market)>) {
    for (pop, market ) in query.iter() {
        
    }
}