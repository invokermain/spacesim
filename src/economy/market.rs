use std::collections::{VecDeque, HashMap};

use super::{components::{CommodityArr, COMMODITY_COUNT, OnPlanet}, events::CommodityProducedEvent};
use bevy::prelude::{Component, EventReader, Query, Entity};

// COMPONENT
#[derive(Component)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub demand: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
    pub market_members: Vec<Entity>,
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
        }
    }
}

impl Market {
    pub fn update_supply(&mut self, changes: &CommodityArr<f32>) {
        for (idx, change) in changes.iter().enumerate() {
            self.total_supply[idx] += change;
        }
    }
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
