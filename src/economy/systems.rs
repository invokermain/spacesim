use bevy::{
    prelude::{warn, Entity, ParamSet, Query, Res, With},
    time::Time,
};

use crate::economy::components::CommodityType;

use super::{
    components::{
        CommodityPricing, CommodityStorage, IsCompany, OnPlanet, OwnedFactories, Population,
        Production, Wealth,
    },
    market::{Market, Transaction},
};

pub fn company_simulate(
    mut q_company: Query<
        (
            Entity,
            &mut Wealth,
            &mut CommodityStorage,
            &OwnedFactories,
            &OnPlanet,
        ),
        With<IsCompany>,
    >,
    mut q_market: Query<&mut Market>,
    q_manufactory: Query<&Production>,
    time: Res<Time>,
) {
    for (company_entity, mut wealth, mut storage, owned_factories, on_planet) in
        q_company.iter_mut()
    {
        let mut market = q_market
            .get_component_mut::<Market>(on_planet.value)
            .unwrap();
        if storage.available_capacity > 0.0 {
            // get everything that we can produce
            let all_producable = owned_factories
                .value
                .iter()
                .map(|&entity| {
                    (
                        entity,
                        q_manufactory.get_component::<Production>(entity).unwrap(),
                    )
                })
                .collect::<Vec<(Entity, &Production)>>();

            // TODO: calculate expected profit for each commodity
            //       for each commodity get market price - manufacture cost,
            //       then order by expected profit.

            for (mantufactory_entity, producable) in all_producable {
                let units = f32::min(producable.output_per_tick, storage.available_capacity);
                let cost = units * producable.cost_per_unit;

                // we can produce something
                if wealth.value > cost {
                    let transaction = Transaction {
                        buyer: company_entity,
                        seller: mantufactory_entity,
                        commodity_type: producable.commodity_type,
                        units,
                        unit_price: producable.cost_per_unit,
                    };
                    let result = market.produce_for_market(
                        &transaction,
                        storage.as_mut(),
                        wealth.as_mut(),
                        &time,
                    );
                    if let Err(msg) = result {
                        warn!("produce for market failed: {:?}", msg);
                    }
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
    r_time: Res<Time>,
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
                            let mut param_set = p_company.p1();
                            let (mut storage, mut wealth) =
                                param_set.get_mut(quote.seller).unwrap();
                            if let Err(msg) = market.consume_from_market(
                                &quote.to_transaction(entity),
                                &mut storage,
                                &mut wealth,
                                &r_time,
                            ) {
                                warn!("Error making transaction: '{}'", msg);
                            };
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
