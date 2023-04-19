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

// `HealthQueryItem` is only available when accessing the query with mutable methods.
// impl<'w> HealthQueryItem<'w> {
//     fn damage(&mut self, value: f32) {
//         self.health.0 -= value;
//     }

//     fn total(&self) -> f32 {
//         self.health.0 + self.buff.as_deref().map_or(0.0, |Buff(buff)| *buff)
//     }
// }
