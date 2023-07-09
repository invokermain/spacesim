use bevy::{ecs::query::WorldQuery, prelude::Entity};

use super::components::{CommodityStorage, Wealth};

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
}

#[derive(WorldQuery)]
pub struct MarketSellerQuery {
    pub entity: Entity,
    pub storage: &'static CommodityStorage,
    pub wealth: &'static Wealth,
}
