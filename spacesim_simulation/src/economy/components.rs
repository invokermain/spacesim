use bevy::prelude::{Bundle, Component, Entity};
use std::fmt::Debug;

use crate::common::marker_components::{IsCompany, IsPlanet};

use super::{
    commodity_type::{CommodityArr, CommodityType, COMMODITY_COUNT},
    market::Market,
};

#[derive(Component, Clone, Copy)]
pub struct Production {
    pub commodity_type: CommodityType,
    pub cost_per_unit: f32,
    pub output_per_tick: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CommodityStorage {
    pub storage: CommodityArr<f32>,
    pub max_capacity: f32,
    pub available_capacity: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CommodityPricing {
    pub value: CommodityArr<f32>,
}

impl Default for CommodityPricing {
    fn default() -> Self {
        Self {
            value: [1.0, 0.5, 2.0],
        }
    }
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
        return units <= self.available_capacity;
    }

    pub fn can_remove(&self, commodity_type: CommodityType, units: f32) -> bool {
        return units <= self.storage[commodity_type as usize];
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

#[derive(Component, Clone, Copy)]
pub struct Population {
    pub consumption: CommodityArr<f32>,
}

#[derive(Component, Clone, Copy)]
pub struct OnPlanet {
    pub value: Entity,
}

#[derive(Component, Clone)]
pub struct OwnedFactories {
    pub value: Vec<Entity>,
}

impl Default for OwnedFactories {
    fn default() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ManufactoryBundle {
    pub production: Production,
    pub on_planet: OnPlanet,
}

#[derive(Bundle)]
pub struct CompanyBundle {
    pub is: IsCompany,
    pub wealth: Wealth,
    pub commodity_storage: CommodityStorage,
    pub commodity_pricing: CommodityPricing,
    pub owned_factories: OwnedFactories,
    pub on_planet: OnPlanet,
}

impl CompanyBundle {
    pub fn new(wealth: f32, owned_factories: &Vec<Entity>, on_planet: Entity) -> Self {
        CompanyBundle {
            is: IsCompany {},
            wealth: Wealth { value: wealth },
            commodity_storage: CommodityStorage::new(100.0),
            commodity_pricing: CommodityPricing::default(),
            owned_factories: OwnedFactories {
                value: owned_factories.clone(),
            },
            on_planet: OnPlanet { value: on_planet },
        }
    }
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub market: Market,
    pub population: Population,
    pub is: IsPlanet,
}
