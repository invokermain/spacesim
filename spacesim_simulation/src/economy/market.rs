use std::collections::VecDeque;

use super::commodity_type::{CommodityArr, CommodityType, COMMODITY_COUNT};

use crate::economy::components::{CommodityStorage, Wealth};
use bevy::prelude::{Component, Entity};

const TRANSACTION_HISTORY_LENGTH: usize = 256;
const MARKET_FORCES_HISTORY_LENGTH: usize = 32;

// COMPONENT
#[derive(Component, Clone)]
pub struct Market {
    // The market serves as an abstraction over a collection of economic entities. It is responsible
    // for tracking macroeconomic values such as demand.
    pub total_supply: CommodityArr<f32>,
    // Market-wide commodity price multiplier
    pub demand_price_modifier: CommodityArr<f32>,

    // a metric representing average supply and demand per tick
    pub supply_pressure: CommodityArr<f32>,
    pub demand_pressure: CommodityArr<f32>,

    // track recent history
    pub transaction_history: VecDeque<(f32, MarketTransaction)>,
    pub total_supply_history: CommodityArr<VecDeque<f32>>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub demand_history: CommodityArr<VecDeque<f32>>,

    // track inter-tick values to be aggregated at end of tick
    pub tick_total_demand: CommodityArr<f32>,
    pub tick_total_supply: CommodityArr<f32>,
}

impl Default for Market {
    fn default() -> Self {
        let market_history_vec = [
            VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
            VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
            VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
        ];
        Self {
            demand_pressure: [0.0; COMMODITY_COUNT],
            supply_pressure: [0.0; COMMODITY_COUNT],
            total_supply: [0.0; COMMODITY_COUNT],
            transaction_history: VecDeque::with_capacity(TRANSACTION_HISTORY_LENGTH),
            demand_price_modifier: [1.0; COMMODITY_COUNT],
            tick_total_demand: [0.0; COMMODITY_COUNT],
            tick_total_supply: [0.0; COMMODITY_COUNT],
            supply_history: market_history_vec.clone(),
            demand_history: market_history_vec.clone(),
            total_supply_history: market_history_vec.clone(),
        }
    }
}

/// Market calculations and core logic etc etc
impl Market {
    pub fn calculate_demand_modifier(&self, commodity_type: CommodityType) -> f32 {
        let supply_pressure = self.supply_pressure[commodity_type as usize]
            / MARKET_FORCES_HISTORY_LENGTH as f32;
        let demand_pressure = self.demand_pressure[commodity_type as usize]
            / MARKET_FORCES_HISTORY_LENGTH as f32;
        let delta = supply_pressure - demand_pressure;
        let supply = self.total_supply[commodity_type as usize];

        // see https://docs.google.com/spreadsheets/d/1_bHLiL4MsosQ6BOG_aq6LiXr3Y9cbwMy-c6p3wxGmtQ/edit#gid=0

        // price increases as supply decreases, the effect is softened with positive supply
        let supply_modifier = (50.0 / (supply + 1.0 + (5.0 * delta).max(0.0))).max(1.0);

        let delta_modifier = 2.0 / (1.0 + (0.5 * delta).exp());

        supply_modifier * delta_modifier
    }

    pub fn unit_price(&self, commodity_type: CommodityType) -> f32 {
        self.demand_price_modifier[commodity_type as usize] * commodity_type.base_price()
    }

    pub fn consume_commodity(&mut self, commodity_type: CommodityType, units: f32) {
        let commodity_idx = commodity_type as usize;
        self.tick_total_demand[commodity_idx] += units;
        self.total_supply[commodity_idx] -= units;
    }

    pub fn produce_commodity(&mut self, commodity_type: CommodityType, units: f32) {
        let commodity_idx = commodity_type as usize;
        self.tick_total_supply[commodity_idx] += units;
        self.total_supply[commodity_idx] += units;
    }

    pub fn purchase_commodity(
        &mut self,
        buyer: Entity,
        buyer_wealth: &mut Wealth,
        buyer_commodity_storage: &mut CommodityStorage,
        commodity_type: CommodityType,
        units: f32,
        timestamp: f32,
    ) -> Result<MarketTransaction, String> {
        let commodity_idx = commodity_type as usize;
        let unit_price = self.unit_price(commodity_type);
        let transaction_cost = unit_price * units;

        if self.total_supply[commodity_idx] < units {
            return Err(
                format!(
                    "Market does not have enough units of {:?}, requested {:.2} but only {:.2} available",
                    commodity_type,
                    units,
                    self.total_supply[commodity_idx]
                ).into()
            );
        }

        if transaction_cost > buyer_wealth.value {
            return Err(
                format!(
                    "Buyer does not have the required funds for purchase, required {:.2} but only {:.2} available",
                    transaction_cost,
                    buyer_wealth.value
                ).into()
            );
        }

        if !buyer_commodity_storage.can_store(units) {
            return Err(
                format!(
                    "Buyer does not have enough storage space for purchase, required {:.2} but only {:.2} available",
                    transaction_cost,
                    buyer_wealth.value
                ).into()
            );
        }

        self.total_supply[commodity_idx] -= units;
        self.tick_total_demand[commodity_idx] += units;
        buyer_commodity_storage.store(commodity_type, units);
        buyer_wealth.value -= transaction_cost;

        let transaction = MarketTransaction {
            transaction_entity: buyer,
            transaction_type: TranscationType::Purchase,
            transaction_cost,
            commodity_type,
            units,
            unit_price,
        };

        self.add_transaction_to_history(timestamp, &transaction);

        Ok(transaction)
    }

    pub fn add_transaction_to_history(
        &mut self,
        timestamp: f32,
        transaction: &MarketTransaction,
    ) {
        self.transaction_history
            .push_front((timestamp, transaction.clone()));

        if self.transaction_history.len() > TRANSACTION_HISTORY_LENGTH {
            self.transaction_history.pop_back();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MarketTransaction {
    pub transaction_entity: Entity,
    pub transaction_type: TranscationType,
    pub transaction_cost: f32,
    pub commodity_type: CommodityType,
    pub units: f32,
    pub unit_price: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum TranscationType {
    /// Commodity was purchased from the Market
    Purchase,
    /// Commodity was sold to the Market
    Sale,
}
