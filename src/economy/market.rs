use std::collections::VecDeque;

use super::components::{
    CommodityArr, CommodityPricing, CommodityStorage, CommodityType, Wealth, COMMODITY_COUNT,
};
use bevy::{
    prelude::{Component, Entity, Query, Res},
    time::Time,
};

const MAX_HISTORY_LENGTH: usize = 256;

// COMPONENT
#[derive(Component, Clone)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub supply_pressure: CommodityArr<f32>,
    pub demand_pressure: CommodityArr<f32>,
    pub demand_price_modifier: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
    pub market_members: Vec<Entity>,
    pub transaction_history: VecDeque<(f32, Transaction)>,
    pub production_history: VecDeque<(f32, Transaction)>,
    pub consumption_history: VecDeque<(f32, Transaction)>,
}

impl Default for Market {
    fn default() -> Self {
        Self {
            demand_pressure: [0.0; COMMODITY_COUNT],
            supply_pressure: [0.0; COMMODITY_COUNT],
            total_supply: [0.0; COMMODITY_COUNT],
            market_members: Vec::new(),
            supply_history: [
                VecDeque::from_iter(vec![0.0; MAX_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MAX_HISTORY_LENGTH]),
                VecDeque::from_iter(vec![0.0; MAX_HISTORY_LENGTH]),
            ],
            transaction_history: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            production_history: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            consumption_history: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            demand_price_modifier: [1.0; COMMODITY_COUNT],
        }
    }
}

impl Market {
    fn calculate_demand_modifier(&self, commodity_type: CommodityType) -> f32 {
        let supply_pressure = self.supply_pressure[commodity_type as usize] / 256.0;
        let demand_pressure = self.demand_pressure[commodity_type as usize] / 256.0;

        if demand_pressure > supply_pressure {
            // markup up to 100x depending as stock dwindles and demand pressure increases.
            // mod roughly more than 100.0 then no markup
            // as mod approaches zero then markup approaches 100
            let modifier =
                self.total_supply[commodity_type as usize] / (demand_pressure - supply_pressure);
            return f32::max(100.0 - 0.28 * modifier.powf(0.28), 1.0);
        } else if supply_pressure > demand_pressure {
            // discount up to 0.75x as stock grows and supply pressure grows
            // mod less than 50 then no discount
            // mod greater than 150 maximum discount
            let modifier =
                self.total_supply[commodity_type as usize] * supply_pressure / demand_pressure;
            return ((450.0 - modifier) / 400.0).clamp(0.75, 1.0);
        } else {
            return 1.0;
        }
    }

    pub fn posit_purchase(
        &self,
        commodity_type: CommodityType,
        units: f32,
        query: &Query<&CommodityStorage, &CommodityPricing>,
    ) -> Option<MarketPurchaseQuote> {
        if self.total_supply[commodity_type as usize] == 0.0 || self.market_members.len() == 0 {
            return None;
        };
        let mut unfulfilled_units = units;
        let mut fulfilled_units = 0.0;
        let mut total_cost = 0.0;
        let mut quoted_transactions = Vec::new();

        for member in &self.market_members {
            // Currently we determine price via a global market price. However in future
            // we could delegate price decisions to the market member, e.g. base on
            // opinion of player.
            let member_storage = query.get_component::<CommodityStorage>(*member).unwrap();
            let commodity_pricing = query.get_component::<CommodityPricing>(*member).unwrap();
            let in_stock = member_storage.storage[commodity_type as usize];
            let fulfillable_units = f32::min(in_stock, unfulfilled_units);
            let unit_price = self.demand_price_modifier[commodity_type as usize]
                * commodity_pricing.value[commodity_type as usize];

            let transaction = PurchaseQuote {
                seller: member.clone(),
                commodity_type,
                units: fulfillable_units,
                unit_price,
            };

            quoted_transactions.push(transaction);

            total_cost += unit_price * fulfillable_units;
            fulfilled_units += fulfillable_units;
            unfulfilled_units -= unfulfilled_units;

            if unfulfilled_units == 0.0 {
                break;
            }
        }

        Some(MarketPurchaseQuote {
            commodity_type,
            units: fulfilled_units,
            unit_cost: total_cost / fulfilled_units,
            quoted_transactions,
        })
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
        let transaction_total_cost = transaction.unit_price * transaction.units;

        // validate that the transaction can go ahead
        if !self.market_members.contains(&transaction.buyer) {
            return Err("Buyer is not a member of this market".into());
        }
        if self.market_members.contains(&transaction.seller) {
            return Err("Seller is a member of this market".into());
        }
        if !seller_storage.can_remove(transaction.commodity_type, transaction.units) {
            return Err(format!(
                "Seller does not have {:.2} units of {:?} available",
                transaction.units, transaction.commodity_type
            )
            .into());
        }
        if !buyer_storage.can_store(transaction.units) {
            return Err(format!(
                "Buyer does not have room to store {:.2} units of {:?} available",
                transaction.units, transaction.commodity_type
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

        seller_storage.remove(transaction.commodity_type, transaction.units);
        buyer_storage.store(transaction.commodity_type, transaction.units);
        buyer_wealth.value -= transaction_total_cost;
        seller_wealth.value += transaction_total_cost;

        self.transaction_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.transaction_history.len() > MAX_HISTORY_LENGTH {
            self.transaction_history.pop_back();
        }

        self.update_market_meta(transaction.commodity_type, transaction.units);

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
        let transaction_total_cost = transaction.unit_price * transaction.units;

        // validate that the transaction can go ahead
        if !self.market_members.contains(&transaction.seller) {
            return Err("Seller is not a member of this market".into());
        }
        if self.market_members.contains(&transaction.buyer) {
            return Err("Buyer is a member of this market".into());
        }
        if !seller_storage.can_remove(transaction.commodity_type, transaction.units) {
            return Err(format!(
                "Seller does not have {:.2} units of {:?} available",
                transaction.units, transaction.commodity_type
            )
            .into());
        }
        if !buyer_storage.can_store(transaction.units) {
            return Err(format!(
                "Buyer does not have room to store {:.2} units of {:?} available",
                transaction.units, transaction.commodity_type
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

        seller_storage.remove(transaction.commodity_type, transaction.units);
        buyer_storage.store(transaction.commodity_type, transaction.units);
        buyer_wealth.value -= transaction_total_cost;
        seller_wealth.value += transaction_total_cost;

        self.transaction_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.transaction_history.len() > MAX_HISTORY_LENGTH {
            self.transaction_history.pop_back();
        }

        self.update_market_meta(transaction.commodity_type, -transaction.units);

        Ok(())
    }

    /// This makes a transaction that does not mutate the 'seller'. e.g. the seller
    /// is considered to be an infinite or out-of-market resource.
    pub fn produce(
        &mut self,
        transaction: &Transaction,
        buyer_storage: &mut CommodityStorage,
        buyer_wealth: &mut Wealth,
        time: &Res<Time>,
    ) -> Result<(), String> {
        let transaction_total_cost = transaction.unit_price * transaction.units;

        // validate that the transaction can go ahead
        if !buyer_storage.can_store(transaction.units) {
            return Err(format!(
                "Buyer does not have {:.2} units of free space available",
                transaction.units
            )
            .into());
        }

        buyer_storage.store(transaction.commodity_type, transaction.units);
        buyer_wealth.value -= transaction_total_cost;

        self.production_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.production_history.len() > MAX_HISTORY_LENGTH {
            self.production_history.pop_back();
        }

        self.update_market_meta(transaction.commodity_type, transaction.units);

        Ok(())
    }

    /// This makes a transaction that does not mutate the 'buyer'. e.g. the buyer
    /// is considered to be an infinite or out-of-market resource.
    pub fn consume(
        &mut self,
        transaction: &Transaction,
        seller_storage: &mut CommodityStorage,
        seller_wealth: &mut Wealth,
        time: &Res<Time>,
    ) -> Result<(), String> {
        let transaction_total_cost = transaction.unit_price * transaction.units;

        // validate that the transaction can go ahead
        if !seller_storage.can_remove(transaction.commodity_type, transaction.units) {
            return Err(format!(
                "Seller does not have {:.2} units of {:?} available",
                transaction.units, transaction.commodity_type
            )
            .into());
        }

        seller_storage.remove(transaction.commodity_type, transaction.units);
        seller_wealth.value += transaction_total_cost;

        self.consumption_history
            .push_front((time.elapsed_seconds(), transaction.clone()));

        if self.consumption_history.len() > MAX_HISTORY_LENGTH {
            self.consumption_history.pop_back();
        }

        self.update_market_meta(transaction.commodity_type, -transaction.units);

        Ok(())
    }

    fn update_market_meta(&mut self, commodity_type: CommodityType, units: f32) {
        self.total_supply[commodity_type as usize] += units;

        let last = self.supply_history[commodity_type as usize]
            .pop_back()
            .unwrap();
        if last > 0.0 {
            self.supply_pressure[commodity_type as usize] -= last;
        } else if last < 0.0 {
            self.demand_pressure[commodity_type as usize] += last;
        }

        self.supply_history[commodity_type as usize].push_front(units);
        if units > 0.0 {
            self.supply_pressure[commodity_type as usize] += units;
        } else if units < 0.0 {
            self.demand_pressure[commodity_type as usize] -= units;
        }

        self.demand_price_modifier[commodity_type as usize] =
            self.calculate_demand_modifier(commodity_type);
    }
}

#[derive(Clone, Debug)]
pub struct PurchaseQuote {
    pub seller: Entity,
    pub commodity_type: CommodityType,
    pub units: f32,
    pub unit_price: f32,
}

impl PurchaseQuote {
    pub fn to_transaction(&self, entity: Entity) -> Transaction {
        Transaction {
            buyer: entity.clone(),
            seller: self.seller,
            commodity_type: self.commodity_type,
            units: self.units,
            unit_price: self.unit_price,
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

#[derive(Clone, Debug)]
pub struct MarketPurchaseQuote {
    pub commodity_type: CommodityType,
    pub units: f32,
    pub unit_cost: f32,
    pub quoted_transactions: Vec<PurchaseQuote>,
}
