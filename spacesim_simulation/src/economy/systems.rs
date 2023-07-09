use crate::economy::commodity_type::CommodityType;
use crate::economy::components::{CommodityConsumer, TargetMarket};
use bevy::prelude::{Query, Res, Time};
use strum::IntoEnumIterator;

use super::{components::CommodityProducer, market::Market};

/// Companies check whether they have money/storage and if so make their Factories to produce goods
/// for them.
pub fn produce_commodities(
    q_producer: Query<(&CommodityProducer, &TargetMarket)>,
    mut q_market: Query<&mut Market>,
    r_time: Res<Time>,
) {
    for (producer, target_market) in q_producer.iter() {
        let mut market = q_market
            .get_component_mut::<Market>(target_market.value)
            .unwrap();

        for (commodity_idx, production_rate) in producer.production.iter().enumerate() {
            if *production_rate > 0.0 {
                market.produce_commodity(
                    commodity_idx.into(),
                    production_rate * r_time.delta_seconds(),
                );
            }
        }
    }
}

pub fn consume_commodities(
    q_consumer: Query<(&CommodityConsumer, &TargetMarket)>,
    mut q_market: Query<&mut Market>,
) {
    for (pop, target_market) in q_consumer.iter() {
        let mut market = q_market
            .get_component_mut::<Market>(target_market.value)
            .unwrap();

        for (commodity_idx, consumption_rate) in pop.consumption.iter().enumerate() {
            if *consumption_rate > 0.0 {
                market.consume_commodity(commodity_idx.into(), *consumption_rate);
            }
        }
    }
}

pub fn update_market_statistics(mut q_market: Query<&mut Market>) {
    for mut market in q_market.iter_mut() {
        for commodity_type in CommodityType::iter() {
            let commodity_idx = commodity_type as usize;

            // update supply metrics
            {
                let last = market.supply_history[commodity_idx].pop_back().unwrap();
                market.supply_pressure[commodity_idx] -= last;

                let tick_total_supply = market.tick_total_supply[commodity_idx];

                market.supply_history[commodity_idx].push_front(tick_total_supply);
                market.supply_pressure[commodity_idx] += tick_total_supply;
            }

            // update demand metrics
            {
                let last = market.demand_history[commodity_idx].pop_back().unwrap();
                market.demand_pressure[commodity_idx] -= last;

                let tick_total_demand = market.tick_total_demand[commodity_idx];

                market.demand_history[commodity_idx].push_front(tick_total_demand);
                market.demand_pressure[commodity_idx] += tick_total_demand;
            }

            market.demand_price_modifier[commodity_idx] =
                market.calculate_demand_modifier(commodity_type);

            market.total_supply_history[commodity_idx].pop_back();
            let total_supply_this_tick = market.total_supply[commodity_idx];
            market.total_supply_history[commodity_idx].push_front(total_supply_this_tick);

            // reset per tick trackers
            market.tick_total_supply[commodity_idx] = 0.0;
            market.tick_total_demand[commodity_idx] = 0.0;
        }
    }
}
