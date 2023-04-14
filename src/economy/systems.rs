use bevy::prelude::{warn, Entity, EventWriter, ParamSet, Query, With};

use crate::economy::components::CommodityType;

use super::{
    components::{
        CommodityPricing, CommodityStorage, IsCompany, OwnedFactories, Population, Production,
        Wealth,
    },
    events::CommodityProducedEvent,
    market::Market,
};

pub fn company_simulate(
    mut q_company: Query<
        (Entity, &mut Wealth, &mut CommodityStorage, &OwnedFactories),
        With<IsCompany>,
    >,
    q_manufactory: Query<&Production>,
    mut ev_commodity_produced: EventWriter<CommodityProducedEvent>,
) {
    for (entity, mut wealth, mut storage, owned_factories) in q_company.iter_mut() {
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

pub fn population_consumption(
    mut q_planet_pop: Query<(Entity, &Population, &mut Market)>,
    mut p_company: ParamSet<(
        Query<&CommodityStorage, &CommodityPricing>,
        Query<(&mut CommodityStorage, &mut Wealth)>,
    )>,
) {
    for (entity, pop, mut market) in q_planet_pop.iter_mut() {
        for (commodity_idx, consumption_rate) in pop.consumption.iter().enumerate() {
            if *consumption_rate >= 0.0 {
                match market.posit_purchase(
                    commodity_idx.into(),
                    *consumption_rate,
                    &p_company.p0(),
                ) {
                    Some(quote) => {
                        for quote in quote.quoted_transactions {
                            let result = market.make_transaction(
                                &quote.to_transaction(entity),
                                &mut p_company.p1(),
                            );
                            if let Some(msg) = result.err() {
                                warn!("Error making transaction: '{}'", msg);
                            }
                        }
                    }
                    None => warn!(
                        "Market cannot meet consumption needs for '{:?}'",
                        CommodityType::from(commodity_idx)
                    ),
                }
            }
        }
    }
}
