use bevy::{ecs::query::WorldQuery, prelude::Entity};

use super::components::{CommodityPricing, CommodityStorage, Wealth};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MarketBuyerMutQuery {
    pub entity: Entity,
    pub storage: &'static mut CommodityStorage,
    pub wealth: &'static mut Wealth,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MarketSellerMutQuery {
    pub entity: Entity,
    pub storage: &'static mut CommodityStorage,
    pub wealth: &'static mut Wealth,
    pub pricing: &'static mut CommodityPricing,
}
