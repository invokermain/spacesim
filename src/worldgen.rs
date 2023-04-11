use bevy::prelude::Commands;

use crate::economy::{
    components::{
        make_commodity_arr_from_iter, CommodityStorage, CommodityType, CompanyBundle, OnPlanet,
        PlanetBundle, Population, Production, ProductionInfo, Wealth,
    },
    market::Market,
};

pub fn create_world(mut commands: Commands) {
    let planet_id = commands
        .spawn(PlanetBundle {
            market: Market::default(),
            population: Population {
                consumption: [0.1, 0.1, 0.1],
            },
        })
        .id();

    commands.spawn(CompanyBundle {
        production: Production {
            info: make_commodity_arr_from_iter([(
                CommodityType::Food,
                Some(ProductionInfo {
                    cost_per_unit: 0.5,
                    output_per_tick: 0.1,
                }),
            )]),
        },
        commodity_storage: CommodityStorage::new(100.0),
        wealth: Wealth { amount: 100.0 },
        on_planet: OnPlanet { planet: planet_id },
    });
}
