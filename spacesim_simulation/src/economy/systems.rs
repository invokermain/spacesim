use crate::common::marker_components::IsCompany;
use crate::planet::components::{OnPlanet, Population};
use bevy::prelude::{warn, Query, With};

use super::{
    components::{OwnedFactories, Production},
    market::Market,
    market_wq::{MarketBuyerMutQuery, MarketSellerMutQuery},
};

// TODO: replace OnPlanet with TargetMarket... any company should be able to produce for
//   an arbitrary target market.
/// Companies check whether they have money/storage and if so make their Factories to produce goods
/// for them.
pub fn company_simulate(
    mut q_company: Query<(MarketBuyerMutQuery, &OwnedFactories, &OnPlanet), With<IsCompany>>,
    mut q_market: Query<&mut Market>,
    q_manufactory: Query<&Production>,
) {
    for (mut company, owned_factories, on_planet) in q_company.iter_mut() {
        let mut market = q_market
            .get_component_mut::<Market>(on_planet.value)
            .unwrap();
        if company.storage.available_capacity > 0.0 {
            let mut producable_commodities: Vec<_> = owned_factories
                .value
                .iter()
                .map(|entity| {
                    let prod = q_manufactory.get_component::<Production>(*entity).unwrap();
                    let profit = prod.cost_per_unit
                        * market.purchase_price_history[prod.commodity_type as usize][0];
                    (prod, profit)
                })
                .collect();

            producable_commodities.sort_by(|a, b| f32::total_cmp(&b.1, &a.1));

            for (producable, _) in producable_commodities {
                let units =
                    f32::min(producable.output_per_tick, buyer.storage.available_capacity);
                let cost = units * producable.cost_per_unit;

                // we can produce something
                if company.wealth.value > cost {
                    let result = market.produce(
                        producable.commodity_type,
                        units,
                        producable.cost_per_unit,
                        &mut company,
                    );
                    if let Err(msg) = result {
                        warn!("produce for market failed: {:?}", msg);
                    }
                }

                if company.storage.available_capacity <= 0.01 {
                    break;
                }
            }
        }
    }
}

pub fn population_consumption(
    mut q_planet_pop: Query<(&Population, &mut Market)>,
    mut q_market_member: Query<MarketSellerMutQuery>,
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
    q_market
        .iter_mut()
        .for_each(|mut market| market.aggregate_tick_statistics());
}
