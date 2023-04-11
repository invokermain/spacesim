use std::collections::VecDeque;

use super::components::{CommodityArr, COMMODITY_COUNT};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub demand: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
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
