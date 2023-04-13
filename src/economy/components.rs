use bevy::{
    prelude::{Bundle, Component, Entity},
    utils::HashMap,
};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::market::Market;

pub const COMMODITY_COUNT: usize = 3;
pub type CommodityArr<T> = [T; COMMODITY_COUNT];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash, Clone, Copy)]
pub enum CommodityType {
    Food,
    Water,
    Clothes,
}

pub fn make_commodity_arr_from_iter<T: Default + Debug + Clone>(
    iter: impl IntoIterator<Item = (CommodityType, T)>,
) -> CommodityArr<T> {
    let hash_map: HashMap<CommodityType, T> = HashMap::from_iter(iter);
    CommodityType::iter()
        .map(|commodity_type| {
            hash_map
                .get(&commodity_type)
                .unwrap_or(&T::default())
                .clone()
        })
        .collect::<Vec<T>>()
        .try_into()
        .unwrap()
}

impl TryFrom<i32> for CommodityType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommodityType::Food),
            1 => Ok(CommodityType::Water),
            2 => Ok(CommodityType::Clothes),
            _ => Err("No commodity type matching this index"),
        }
    }
}

impl From<usize> for CommodityType {
    fn from(value: usize) -> Self {
        match value {
            0 => CommodityType::Food,
            1 => CommodityType::Water,
            2 => CommodityType::Clothes,
            _ => panic!("No commodity type matching this index"),
        }
    }
}

#[derive(Component)]
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
            value: Default::default(),
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

    pub fn store(&mut self, commodity_type: CommodityType, units: f32) -> bool {
        if units > self.available_capacity {
            return false;
        }
        self.storage[commodity_type as usize] += units;
        self.available_capacity -= units;
        true
    }
}

#[derive(Component)]
pub struct Wealth {
    pub value: f32,
}

#[derive(Component)]
pub struct Population {
    pub consumption: CommodityArr<f32>,
}

#[derive(Component)]
pub struct OnPlanet {
    pub value: Entity,
}

#[derive(Component)]
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

#[derive(Component)]
pub struct IsCompany {}

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
}
