use std::collections::HashMap;
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
