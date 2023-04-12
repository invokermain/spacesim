use bevy::prelude::Commands;

use crate::economy::{
    components::{
        make_commodity_arr_from_iter, CommodityStorage, CommodityType, ManufactoryBundle, OnPlanet,
        PlanetBundle, Population, Production, ProductionInfo, Wealth, CompanyBundle, ConnectedStorage, OwnedBy,
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

    let company_id = commands
        .spawn(CompanyBundle {
            wealth: Wealth { value: 100.0 },
            commodity_storage: CommodityStorage::new(100.0),
        })
        .id();


    commands.spawn(ManufactoryBundle {
        production: Production {
            info: make_commodity_arr_from_iter([(
                CommodityType::Food,
                Some(ProductionInfo {
                    cost_per_unit: 0.5,
                    output_per_tick: 0.1,
                }),
            )]),
        },
        on_planet: OnPlanet { value: planet_id },
        connected_storage: ConnectedStorage { value: Some(company_id) },
        owned_by: OwnedBy { value: Some(company_id) },
    });
}
