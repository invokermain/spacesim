use bevy::prelude::{Commands, Entity, Query};

use crate::economy::components::{
    make_commodity_arr_from_iter, CommodityStorage, CommodityType, CompanyBundle, Market,
    MarketMember, PlanetBundle, Population, Production, ProductionInfo, Wealth,
};

pub fn create_world(mut commands: Commands) {
    commands.spawn(PlanetBundle {
        market: Market::default(),
        population: Population {
            consumption: [1.0, 1.0, 1.0],
        },
    });
}

pub fn create_companies(mut commands: Commands, mut query: Query<(Entity, &mut Market)>) {
    for (entity, mut market) in query.iter_mut() {
        // Make a Company
        let company_id = commands.spawn(CompanyBundle {
            production: Production {
                info: make_commodity_arr_from_iter([(
                    CommodityType::Food,
                    Some(ProductionInfo {
                        cost_per_unit: 1.0,
                        output_per_tick: 1.0,
                    }),
                )]),
            },
            commodity_storage: CommodityStorage::new(100.0),
            wealth: Wealth { amount: 100.0 },
            market_member: MarketMember { member_of: entity },
        });
        market.add_company(company_id.id());
    }
}
