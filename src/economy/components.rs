use std::collections::{HashSet, VecDeque};

use bevy::{
    prelude::{Bundle, Component, Entity},
    utils::HashMap,
};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Component)]
pub struct Market {
    // The market serves as an abstraction over the various economic components of a
    // Planet. It is responsible for tracking macroeconomic values such as demand.
    pub demand: CommodityArr<f32>,
    pub supply_history: CommodityArr<VecDeque<f32>>,
    pub total_supply: CommodityArr<f32>,
    pub trade_routes: HashSet<u32>,
    pub companies: HashSet<Entity>,
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
            trade_routes: HashSet::new(),
            companies: HashSet::new(),
        }
    }
}

impl Market {
    pub fn add_company(&mut self, entity: Entity) {
        self.companies.insert(entity.clone());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProductionInfo {
    pub cost_per_unit: f32,
    pub output_per_tick: f32,
}

#[derive(Component)]
pub struct Production {
    pub info: CommodityArr<Option<ProductionInfo>>,
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
}

#[derive(Component)]
pub struct Wealth {
    pub amount: f32,
}

#[derive(Component)]
pub struct Population {
    pub consumption: CommodityArr<f32>,
}

#[derive(Component)]
pub struct MarketMember {
    pub member_of: Entity,
}

#[derive(Bundle)]
pub struct CompanyBundle {
    pub production: Production,
    pub commodity_storage: CommodityStorage,
    pub wealth: Wealth,
    pub market_member: MarketMember,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub market: Market,
    pub population: Population,
}

// Entity / Component Tree:
// - Companies: A company exists on a certain Planet and can produce goods.
//     - Production
//     - CommmodityStorage
//     - Wealth
//     - MarketMember
// - Planets:
//     - Market
//     - Population
// - Ships: These can enact trades between Planets via their Markets
//     - Wealth
//     - CommmodityStorage

// Systems:
// - ManufactureSystem: Wealth, Production, Storage, MarketMember
//     Logic: If can afford, produce goods, increase storage, decrease wealth
//     Event: MarketSupplyUpdate(MarketId, Commodity, +Produced)
// - MarketSupplyUpdateSystem: Market, Event: MarketSupplyUpdate
//     Logic: Update Total Supply, Update Supply History, Update Demand
// - PopulationConsumeSystem: Population, Market
//     Logic: split consumption across number of companies, purchase supply from those
//            companies at market price.
//     Event: MarketSupplyUpdate(MarketId, Commodity, -Consumed)
