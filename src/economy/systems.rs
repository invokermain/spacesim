use bevy::{
    prelude::{Entity, EventReader, EventWriter, Query},
    utils::HashMap,
};

use super::{
    components::{CommodityArr, CommodityStorage, OnPlanet, Production, Wealth, COMMODITY_COUNT},
    events::CommodityProducedEvent,
    market::Market,
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
                    if wealth.amount > cost {
                        storage.store(commodity_idx.into(), units);
                        wealth.amount -= cost;

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

pub fn market_supply_update(
    mut ev_commodity_produced: EventReader<CommodityProducedEvent>,
    mut q_market: Query<&mut Market>,
    q_on_planet: Query<&OnPlanet>,
) {
    let mut agg_supply: HashMap<Entity, CommodityArr<f32>> = HashMap::new();
    for event in ev_commodity_produced.iter() {
        let entry = agg_supply
            .entry(event.source_entity)
            .or_insert([0.0; COMMODITY_COUNT]);
        entry[event.commodity_type as usize] += event.change;
    }

    for (entity, changed_supply) in agg_supply.iter() {
        let planet_id = q_on_planet.get(*entity).unwrap().planet;
        let mut market = q_market.get_mut(planet_id).unwrap();
        market.update_supply(changed_supply);
    }
}
