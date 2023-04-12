use bevy::prelude::Commands;

use crate::economy::{
    components::{
        CommodityStorage, CommodityType, CompanyBundle, IsCompany, ManufactoryBundle, OnPlanet,
        OwnedFactories, PlanetBundle, Population, Production, Wealth,
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

    let manufactory_id = commands
        .spawn(ManufactoryBundle {
            production: Production {
                commodity_type: CommodityType::Food,
                cost_per_unit: 0.5,
                output_per_tick: 0.1,
            },
            on_planet: OnPlanet { value: planet_id },
        })
        .id();

    commands.spawn(CompanyBundle {
        wealth: Wealth { value: 100.0 },
        commodity_storage: CommodityStorage::new(100.0),
        is: IsCompany {},
        owned_factories: OwnedFactories {
            value: vec![manufactory_id],
        },
        on_planet: OnPlanet { value: planet_id },
    });
}
