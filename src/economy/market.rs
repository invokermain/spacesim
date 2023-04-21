use std::collections::VecDeque;

use super::{
    commodity_type::{CommodityArr, CommodityType, COMMODITY_COUNT},
    components::{CommodityStorage, Wealth},
    market_wq::MarketMemberMutQuery,
};
use bevy::{
    prelude::{info, Component, Entity, Query, Res},
    time::Time,
};
use strum::IntoEnumIterator;

const TRANSACTION_HISTORY_LENGTH: usize = 256;
const MARKET_FORCES_HISTORY_LENGTH: usize = 32;

// COMPONENT
#[derive(Component, Clone)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub supply_pressure: CommodityArr<f32>,
    pub demand_pressure: CommodityArr<f32>,
    pub demand_price_modifier: CommodityArr<f32>,
    pub total_supply: CommodityArr<f32>,
    pub market_members: Vec<Entity>,

    pub transaction_history: VecDeque<(f32, Transaction)>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub demand_history: CommodityArr<VecDeque<f32>>,

    // new
    pub tick_total_demand: CommodityArr<f32>,
    pub tick_total_supply: CommodityArr<f32>,
}

impl Default for Market {
    fn default() -> Self {
        Self {
            demand_pressure: [0.0; COMMODITY_COUNT],
            supply_pressure: [0.0; COMMODITY_COUNT],
            total_supply: [0.0; COMMODITY_COUNT],
            market_members: Vec::new(),
            transaction_history: VecDeque::with_capacity(TRANSACTION_HISTORY_LENGTH),
            demand_price_modifier: [1.0; COMMODITY_COUNT],
            tick_total_demand: [0.0; COMMODITY_COUNT],
            tick_total_supply: [0.0; COMMODITY_COUNT],
            supply_history: [
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
            ],
            demand_history: [
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MARKET_FORCES_HISTORY_LENGTH]),
            ],
        }
    }
}

impl Market {
    fn calculate_demand_modifier(&self, commodity_type: CommodityType) -> f32 {
        let supply_pressure = self.supply_pressure[commodity_type as usize];
        let demand_pressure = self.demand_pressure[commodity_type as usize];
        let delta = supply_pressure - demand_pressure;
        let supply = self.total_supply[commodity_type as usize];

        // see https://docs.google.com/spreadsheets/d/1_bHLiL4MsosQ6BOG_aq6LiXr3Y9cbwMy-c6p3wxGmtQ/edit#gid=0

        // price increases as supply decreases, the effect is softened with positive supply
        let supply_modifier = (50.0 / (supply + 1.0 + (5.0 * delta).max(0.0))).max(1.0);

        let delta_modifier = 2.0 / (1.0 + (0.5 * delta).exp());

        supply_modifier * delta_modifier
    }

    pub fn consume(
        &mut self,
        commodity_type: CommodityType,
        units: f32,
        query: &mut Query<MarketMemberMutQuery>,
    ) -> MarketConsumeResult {
        self.tick_total_demand[commodity_type as usize] += units;

        if self.total_supply[commodity_type as usize] <= 0.001 || self.market_members.len() == 0 {
            return MarketConsumeResult {
                commodity_type,
                requested_units: units,
                fulfilled_units: 0.0,
                unit_cost: 0.0,
                total_cost: 0.0,
            };
        };

        let mut unfulfilled_units = units;
        let mut fulfilled_units = 0.0;
        let mut total_cost = 0.0;

        for member_id in &self.market_members {
            // Currently we determine price via a global market price. However in future
            // we could delegate price decisions to the market member, e.g. base on
            // opinion of player.
            let mut market_member = query.get_mut(*member_id).unwrap();

            let in_stock = market_member.storage.storage[commodity_type as usize];

            if in_stock <= 0.001 {
                continue;
            }

            let fulfillable_units = f32::min(in_stock, unfulfilled_units);
            let unit_price = self.demand_price_modifier[commodity_type as usize]
                * market_member.pricing.value[commodity_type as usize];
            let price = unit_price * fulfillable_units;

            market_member.wealth.value += price;
            market_member
                .storage
                .remove(commodity_type, fulfillable_units);

            total_cost += unit_price * fulfillable_units;
            fulfilled_units += fulfillable_units;
            unfulfilled_units -= unfulfilled_units;

            if unfulfilled_units <= 0.001 {
                break;
            }
        }

        self.total_supply[commodity_type as usize] -= units;

        MarketConsumeResult {
            commodity_type,
            requested_units: units,
            fulfilled_units,
            unit_cost: total_cost / fulfilled_units,
            total_cost,
        }
    }

    // This assumes that the seller is in market, and the buyer is out of market.
    pub fn buy(
        &mut self,
        transaction: &Transaction,
        buyer_storage: &mut CommodityStorage,
        buyer_wealth: &mut Wealth,
        seller_storage: &mut CommodityStorage,
        seller_wealth: &mut Wealth,
        time: &Res<Time>,
    ) -> Result<(), String> {
        let Transaction {
            buyer,
            seller,
            commodity_type,
            units,
            unit_price,
        } = transaction;
        let transaction_total_cost = unit_price * units;

        // validate that the transaction can go ahead
        if !self.market_members.contains(buyer) {
            return Err("Buyer is not a member of this market".into());
        }
        if self.market_members.contains(seller) {
            return Err("Seller is a member of this market".into());
        }
        if !seller_storage.can_remove(*commodity_type, *units) {
            return Err(format!(
                "Seller does not have {:.2} units of {:?} available",
                units, commodity_type
            )
            .into());
        }
        if !buyer_storage.can_store(*units) {
            return Err(format!(
                "Buyer does not have room to store {:.2} units of {:?} available",
                units, commodity_type
            )
            .into());
        }
        if buyer_wealth.value < transaction_total_cost {
            return Err(format!(
                "Buyer cannot afford {:.2} cost, they only have {:.2} available",
                transaction_total_cost, buyer_wealth.value
            )
            .into());
        }

        seller_storage.remove(*commodity_type, *units);
        buyer_storage.store(*commodity_type, *units);
        buyer_wealth.value -= transaction_total_cost;
        seller_wealth.value += transaction_total_cost;

        self.transaction_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.transaction_history.len() > TRANSACTION_HISTORY_LENGTH {
            self.transaction_history.pop_back();
        }

        self.total_supply[*commodity_type as usize] += units;

        Ok(())
    }

    // This assumes that the seller is in market, and the buyer is out of market.
    pub fn sell(
        &mut self,
        transaction: &Transaction,
        buyer_storage: &mut CommodityStorage,
        buyer_wealth: &mut Wealth,
        seller_storage: &mut CommodityStorage,
        seller_wealth: &mut Wealth,
        time: &Res<Time>,
    ) -> Result<(), String> {
        let Transaction {
            buyer,
            seller,
            commodity_type,
            units,
            unit_price,
        } = transaction;
        let transaction_total_cost = unit_price * units;

        // validate that the transaction can go ahead
        if !self.market_members.contains(seller) {
            return Err("Seller is not a member of this market".into());
        }
        if self.market_members.contains(buyer) {
            return Err("Buyer is a member of this market".into());
        }
        if !seller_storage.can_remove(*commodity_type, *units) {
            return Err(format!(
                "Seller does not have {:.2} units of {:?} available",
                units, commodity_type
            )
            .into());
        }
        if !buyer_storage.can_store(*units) {
            return Err(format!(
                "Buyer does not have room to store {:.2} units of {:?} available",
                units, commodity_type
            )
            .into());
        }
        if buyer_wealth.value < transaction_total_cost {
            return Err(format!(
                "Buyer cannot afford {:.2} cost, they only have {:.2} available",
                transaction_total_cost, buyer_wealth.value
            )
            .into());
        }

        seller_storage.remove(*commodity_type, *units);
        buyer_storage.store(*commodity_type, *units);
        buyer_wealth.value -= transaction_total_cost;
        seller_wealth.value += transaction_total_cost;

        self.transaction_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.transaction_history.len() > TRANSACTION_HISTORY_LENGTH {
            self.transaction_history.pop_back();
        }

        self.total_supply[*commodity_type as usize] -= units;

        Ok(())
    }

    /// This makes a transaction that does not mutate the 'seller'. e.g. the seller
    /// is considered to be an infinite or out-of-market resource.
    pub fn produce(
        &mut self,
        commodity_type: CommodityType,
        units: f32,
        unit_price: f32,
        buyer_storage: &mut CommodityStorage,
        buyer_wealth: &mut Wealth,
    ) -> Result<(), String> {
        self.tick_total_supply[commodity_type as usize] += units;
        let transaction_total_cost = unit_price * units;

        // validate that the transaction can go ahead
        if !buyer_storage.can_store(units) {
            return Err(format!(
                "Buyer does not have {:.2} units of free space available",
                units
            )
            .into());
        }

        buyer_storage.store(commodity_type, units);
        buyer_wealth.value -= transaction_total_cost;

        self.total_supply[commodity_type as usize] += units;

        Ok(())
    }

    pub fn aggregate_tick_statistics(&mut self) {
        for commodity_type in CommodityType::iter() {
            let commodity_idx = commodity_type as usize;

            // update supply metrics
            {
                let last = self.supply_history[commodity_idx].pop_back().unwrap();
                self.supply_pressure[commodity_idx] -= last;

                let tick_total_supply =
                    self.tick_total_supply[commodity_idx] / MARKET_FORCES_HISTORY_LENGTH as f32;

                self.supply_history[commodity_idx].push_front(tick_total_supply);
                self.supply_pressure[commodity_idx] += tick_total_supply;
            }

            // update demand metrics
            {
                let last = self.demand_history[commodity_idx].pop_back().unwrap();
                self.demand_pressure[commodity_idx] -= last;

                let tick_total_demand =
                    self.tick_total_demand[commodity_idx] / MARKET_FORCES_HISTORY_LENGTH as f32;

                self.demand_history[commodity_idx].push_front(tick_total_demand);
                self.demand_pressure[commodity_idx] += tick_total_demand;
            }

            self.tick_total_supply[commodity_idx] = 0.0;
            self.tick_total_demand[commodity_idx] = 0.0;

            self.demand_price_modifier[commodity_idx] =
                self.calculate_demand_modifier(commodity_type);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub buyer: Entity,
    pub seller: Entity,
    pub commodity_type: CommodityType,
    pub units: f32,
    pub unit_price: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct MarketConsumeResult {
    pub commodity_type: CommodityType,
    pub requested_units: f32,
    pub fulfilled_units: f32,
    pub unit_cost: f32,
    pub total_cost: f32,
}
