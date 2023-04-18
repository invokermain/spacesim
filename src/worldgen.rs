use bevy::prelude::Commands;

use crate::{
    common::marker_components::IsPlanet,
    economy::{
        components::{
            CommodityType, CompanyBundle, ManufactoryBundle, OnPlanet, Population, Production,
            Wealth,
        },
        market::Market,
    },
};

pub fn create_world(mut commands: Commands) {
    // PLANET ONE
    let planet_id = commands
        .spawn((
            Population {
                consumption: [0.4, 0.0, 0.0],
            },
            Wealth {
                value: f32::INFINITY,
            },
            IsPlanet {},
        ))
        .id();

    let manufactory_id = commands
        .spawn(ManufactoryBundle {
            production: Production {
                commodity_type: CommodityType::Food,
                cost_per_unit: 0.5,
                output_per_tick: 1.0,
            },
            on_planet: OnPlanet { value: planet_id },
        })
        .id();

    let company_id = commands
        .spawn(CompanyBundle::new(100.0, &vec![manufactory_id], planet_id))
        .id();

    let mut market = Market::default();
    market.market_members.push(company_id);

    commands.entity(planet_id).insert(market);

    // PLANET TWO
    let planet_id = commands
        .spawn((
            Population {
                consumption: [0.4, 0.0, 0.0],
            },
            Wealth {
                value: f32::INFINITY,
            },
            IsPlanet {},
        ))
        .id();

    let manufactory_id = commands
        .spawn(ManufactoryBundle {
            production: Production {
                commodity_type: CommodityType::Food,
                cost_per_unit: 0.5,
                output_per_tick: 1.0,
            },
            on_planet: OnPlanet { value: planet_id },
        })
        .id();

    let company_id = commands
        .spawn(CompanyBundle::new(100.0, &vec![manufactory_id], planet_id))
        .id();

    let mut market = Market::default();
    market.market_members.push(company_id);

    commands.entity(planet_id).insert(market);
}
