use bevy::prelude::{warn, Entity, Query, With};

use crate::common::marker_components::IsCompany;

use super::{
    components::{CommodityStorage, OnPlanet, OwnedFactories, Population, Production, Wealth},
    market::Market,
    market_wq::MarketMemberMutQuery,
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
) {
    for (company_id, mut wealth, mut storage, owned_factories, on_planet) in q_company.iter_mut() {
        let mut market = q_market
            .get_component_mut::<Market>(on_planet.value)
            .unwrap();
        if storage.available_capacity > 0.0 {
            // TODO: calculate expected profit for each commodity
            //       for each commodity get market price - manufacture cost,
            //       then order by expected profit.

            for manufactory_entity in &owned_factories.value {
                let producable = q_manufactory
                    .get_component::<Production>(*manufactory_entity)
                    .unwrap();
                let units = f32::min(producable.output_per_tick, storage.available_capacity);
                let cost = units * producable.cost_per_unit;

                // we can produce something
                if wealth.value > cost {
                    let result = market.produce(
                        producable.commodity_type,
                        units,
                        producable.cost_per_unit,
                        company_id,
                        storage.as_mut(),
                        wealth.as_mut(),
                    );
                    if let Err(msg) = result {
                        warn!("produce for market failed: {:?}", msg);
                    }
                }

                if storage.available_capacity <= 0.01 {
                    break;
                }
            }
        }
    }
}

pub fn population_consumption(
    mut q_planet_pop: Query<(&Population, &mut Market)>,
    mut q_market_member: Query<MarketMemberMutQuery>,
) {
    for (pop, mut market) in q_planet_pop.iter_mut() {
        for (commodity_idx, consumption_rate) in pop.consumption.iter().enumerate() {
            if *consumption_rate > 0.0 {
                market.consume(
                    commodity_idx.into(),
                    *consumption_rate,
                    &mut q_market_member,
                );
            }
        }
    }
}

pub fn update_market_statistics(mut q_market: Query<&mut Market>) {
    for mut market in q_market.iter_mut() {
        market.aggregate_tick_statistics();
    }
}
