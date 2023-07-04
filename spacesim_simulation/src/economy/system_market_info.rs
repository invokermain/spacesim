use crate::economy::commodity_type::CommodityType;
use crate::economy::market::Market;
use bevy::log::info;
use bevy::prelude::{Entity, Query, ResMut, Resource};
use bevy::utils::hashbrown::HashMap;
use strum::IntoEnumIterator;

#[derive(Resource, Default)]
pub struct SystemMarketInfo {
    pub trade_potentials: Vec<MarketTradePotential>,
    pub market_total_trade_potential: HashMap<Entity, f32>,
}

/// Represents the demand_modifier differential between two Markets. `trade_potential` will always
/// be positive.
#[derive(Copy, Clone)]
pub struct MarketTradePotential {
    pub trade_potential: f32,
    pub positive_market_entity: Entity,
    pub negative_market_entity: Entity,
}

/// Average unit cost of all goods at source vs all goods at target
pub fn update_system_market_info(
    q_markets: Query<(Entity, &Market)>,
    mut res_system_market_info: ResMut<SystemMarketInfo>,
) {
    let mut market_trade_potentials = Vec::new();
    let mut iter = q_markets.iter_combinations();

    while let Some([(entity_a, market_a), (entity_b, market_b)]) = iter.fetch_next() {
        // Trade Potential for each commodity assuming you are buying at A, selling at B.
        let commodity_trade_potential: Vec<f32> = CommodityType::iter()
            .map(|commodity_type| {
                market_b.demand_price_modifier[commodity_type as usize]
                    - market_a.demand_price_modifier[commodity_type as usize]
            })
            .collect();

        let a_is_positive = commodity_trade_potential.iter().sum::<f32>() > 0.0;
        let trade_potential = match a_is_positive {
            true => commodity_trade_potential.iter().filter(|&&x| x > 0.0).sum(),
            false => commodity_trade_potential
                .iter()
                .map(|x| -1.0 * x)
                .filter(|&x| x > 0.0)
                .sum(),
        };

        market_trade_potentials.push(MarketTradePotential {
            trade_potential,
            positive_market_entity: if a_is_positive { entity_a } else { entity_b },
            negative_market_entity: if a_is_positive { entity_b } else { entity_a },
        })
    }

    res_system_market_info.market_total_trade_potential = HashMap::new();
    for trade in &market_trade_potentials {
        let entry = res_system_market_info
            .market_total_trade_potential
            .entry(trade.positive_market_entity)
            .or_insert(0.0);
        *entry += trade.trade_potential;
    }

    res_system_market_info.trade_potentials = market_trade_potentials;
}
