use bevy::ecs::query::WorldQuery;

use super::components::{CommodityPricing, CommodityStorage, Wealth};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MarketSubjectMutQuery {
    pub storage: &'static mut CommodityStorage,
    pub wealth: &'static mut Wealth,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MarketMemberMutQuery {
    pub storage: &'static mut CommodityStorage,
    pub wealth: &'static mut Wealth,
    pub pricing: &'static mut CommodityPricing,
}
