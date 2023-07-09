use bevy::prelude::{Bundle, Component, Entity};
use std::fmt::Debug;

use crate::planet::components::OnPlanet;

use super::commodity_type::{CommodityArr, CommodityType, COMMODITY_COUNT};

#[derive(Component, Clone, Copy)]
pub struct CommodityProducer {
    pub production: CommodityArr<f32>,
}

#[derive(Component, Clone, Copy)]
pub struct CommodityConsumer {
    pub consumption: CommodityArr<f32>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CommodityStorage {
    pub storage: CommodityArr<f32>,
    pub max_capacity: f32,
    pub available_capacity: f32,
}

impl CommodityStorage {
    pub fn new(max_capacity: f32) -> Self {
        CommodityStorage {
            storage: [0.0; COMMODITY_COUNT],
            max_capacity,
            available_capacity: max_capacity,
        }
    }

    pub fn can_store(&self, units: f32) -> bool {
        units <= self.available_capacity
    }

    pub fn can_remove(&self, commodity_type: CommodityType, units: f32) -> bool {
        units <= self.storage[commodity_type as usize]
    }

    pub fn store(&mut self, commodity_type: CommodityType, units: f32) {
        self.storage[commodity_type as usize] += units;
        self.available_capacity -= units;
    }

    pub fn remove(&mut self, commodity_type: CommodityType, units: f32) {
        self.storage[commodity_type as usize] -= units;
        self.available_capacity += units;
    }
}

#[derive(Component, Clone, Copy)]
pub struct Wealth {
    pub value: f32,
}

#[derive(Bundle)]
pub struct ManufactoryBundle {
    pub production: CommodityProducer,
    pub on_planet: OnPlanet,
    pub target_market: TargetMarket,
}

#[derive(Component)]
pub struct TargetMarket {
    pub value: Entity,
}
