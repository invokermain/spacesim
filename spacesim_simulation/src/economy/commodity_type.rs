use std::fmt::Debug;

use strum_macros::{Display, EnumIter};

pub const COMMODITY_COUNT: usize = 3;
pub type CommodityArr<T> = [T; COMMODITY_COUNT];

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash, Clone, Copy)]
pub enum CommodityType {
    Food,
    Water,
    Clothes,
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

impl CommodityType {
    pub fn base_price(&self) -> f32 {
        match self {
            CommodityType::Food => 0.5,
            CommodityType::Water => 0.3,
            CommodityType::Clothes => 1.5,
        }
    }
}
