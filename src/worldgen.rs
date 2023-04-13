use bevy::prelude::Commands;

use crate::economy::{
    components::{
        CommodityType, CompanyBundle, ManufactoryBundle, OnPlanet, PlanetBundle, Population,
        Production,
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

    commands.spawn(CompanyBundle::new(100.0, &vec![manufactory_id], planet_id));
}
