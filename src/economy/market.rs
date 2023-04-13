use std::collections::{HashMap, VecDeque};

use super::{
    components::{
        CommodityArr, CommodityPricing, CommodityStorage, CommodityType, OnPlanet, COMMODITY_COUNT, Wealth,
    },
    events::CommodityProducedEvent,
};
use bevy::prelude::{Component, Entity, EventReader, Query};

// COMPONENT
#[derive(Component)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub demand: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
    pub market_members: Vec<Entity>,
    pub transaction_history: VecDeque<CommitedTransaction>,
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
        quantity: f32,
        query: &Query<&CommodityStorage, &CommodityPricing>,
    ) -> Option<MarketPurchaseQuote> {
        if self.total_supply[commodity_type.into()] == 0.0 {
            return None;
        };
        let mut unfulfilled_quantity = quantity;
        let mut fulfilled_quantity = 0.0;
        let mut total_cost = 0.0;
        let mut quoted_transactions = Vec::new();

        for member in self.market_members {
            // Currently we determine price via a global market price. However in future we could delegate price decisions
            // to the market member, e.g. base on opinion of player.
            let member_storage = query.get_component::<CommodityStorage>(member).unwrap();
            let commodity_pricing = query.get_component::<CommodityPricing>(member).unwrap();
            let in_stock = member_storage.storage[commodity_type as usize];
            let fulfillable_quantity = f32::max(in_stock, unfulfilled_quantity);
            let cost = self.demand_modifier() * commodity_pricing.value[commodity_type as usize];

            quoted_transactions.push(QuoteTransaction {
                seller: member.clone(),
                commodity_type,
                quantity: fulfillable_quantity,
                cost,
                unit_cost: cost / fulfillable_quantity,
            });

            total_cost += cost;
            fulfillable_quantity += fulfillable_quantity;
            unfulfilled_quantity -= unfulfilled_quantity;

            if unfulfilled_quantity == 0.0 {
                break;
            }
        }

        Some(MarketPurchaseQuote {
            commodity_type,
            quantity: fulfilled_quantity,
            cost: total_cost,
            unit_cost: total_cost / fulfilled_quantity,
            quoted_transactions,
        })
    }

    pub fn make_transaction(
        transaction: &RequestTransaction,
        mut query: Query<(&mut CommodityStorage, &mut Wealth)>
    ) -> Result<CommittedTransaction, String> {
        let mut seller_storage = query.get_component::<CommodityStorage>(transaction.seller).unwrap();
        let mut buyer_storage = query.get_component::<CommodityStorage>(transaction.buyer).unwrap();

        

        Err("foo".into())
    }

    pub fn make_transaction_many(
        transactions: Vec<RequestTransaction>,
    ) -> Vec<Result<CommittedTransaction, String>> {
        None
    }
}

pub struct CommittedTransaction {
    seller: Entity,
    buyer: Entity,
    commodity_type: CommodityType,
    quantity: f32,
    unit_cost: f32,
}

pub struct QuoteTransaction {
    seller: Entity,
    commodity_type: CommodityType,
    quantity: f32,
    unit_price: f32
}

pub struct RequestTransaction {
    buyer: Entity,
    seller: Entity,
    commodity_type: CommodityType,
    quantity: f32,
    unit_price: f32,
    acceptable_margin: f32,
}

pub struct MarketPurchaseQuote {
    commodity_type: CommodityType,
    quantity: f32,
    cost: f32,
    unit_cost: f32,
    quoted_transactions: Vec<QuoteTransaction>,
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
