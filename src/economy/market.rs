use std::collections::{HashMap, VecDeque};

use super::{
    components::{
        CommodityArr, CommodityPricing, CommodityStorage, CommodityType, OnPlanet, Wealth,
        COMMODITY_COUNT,
    },
    events::CommodityProducedEvent,
};
use bevy::prelude::{info, Component, Entity, EventReader, Query};

// COMPONENT
#[derive(Component)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub demand: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
    pub market_members: Vec<Entity>,
    pub transaction_history: VecDeque<Transaction>,
}

impl Default for Market {
    fn default() -> Self {
        Self {
            demand: [0.0; COMMODITY_COUNT],
            supply_history: [
                VecDeque::with_capacity(256),
                VecDeque::with_capacity(256),
                VecDeque::with_capacity(256),
            ],
            total_supply: [0.0; COMMODITY_COUNT],
            market_members: Vec::new(),
            transaction_history: VecDeque::new(),
        }
    }
}

impl Market {
    pub fn update_supply(&mut self, changes: &CommodityArr<f32>) {
        for (idx, change) in changes.iter().enumerate() {
            self.total_supply[idx] += change;
        }
    }

    fn demand_modifier(&self) -> f32 {
        1.0
    }

    pub fn posit_purchase(
        &self,
        commodity_type: CommodityType,
        units: f32,
        query: &Query<&CommodityStorage, &CommodityPricing>,
    ) -> Option<MarketPurchaseQuote> {
        if self.total_supply[commodity_type as usize] == 0.0 {
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
            let fulfillable_units = f32::max(in_stock, unfulfilled_units);
            let cost = self.demand_modifier() * commodity_pricing.value[commodity_type as usize];

            quoted_transactions.push(PurchaseQuote {
                seller: member.clone(),
                commodity_type,
                units: fulfillable_units,
                unit_price: cost / fulfillable_units,
            });

            total_cost += cost;
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

    pub fn make_transaction(
        &mut self,
        transaction: &Transaction,
        query: &mut Query<(&mut CommodityStorage, &mut Wealth)>,
    ) -> Result<(), String> {
        let [(mut seller_storage, mut seller_wealth), (mut buyer_storage, mut buyer_wealth)] =
            query
                .get_many_mut([transaction.seller, transaction.buyer])
                .unwrap();
        let transaction_total_cost = transaction.unit_price * transaction.units;

        // validate that the transaction can go ahead
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

        self.transaction_history.push_front(transaction.clone());

        info!("Transaction made: {:?}", transaction);

        Ok(())
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct MarketPurchaseQuote {
    pub commodity_type: CommodityType,
    pub units: f32,
    pub unit_cost: f32,
    pub quoted_transactions: Vec<PurchaseQuote>,
}

// SYSTEMS
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
        let planet_id = q_on_planet.get(*entity).unwrap().value;
        let mut market = q_market.get_mut(planet_id).unwrap();
        market.update_supply(changed_supply);
    }
}
