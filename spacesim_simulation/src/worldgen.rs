use bevy::prelude::Commands;

use crate::economy::{
    commodity_type::CommodityType,
    components::{CompanyBundle, ManufactoryBundle, Production},
};
use crate::planet::components::{OnPlanet, PlanetBundle, Population};
use crate::ships::bundles::ShipBundle;
use crate::ships::components::SystemCoordinates;

pub fn create_world(mut commands: Commands) {
    // PLANET: Earth
    {
        let planet_earth_id = commands.spawn_empty().id();

        let food_factory_id = commands
            .spawn(ManufactoryBundle {
                production: Production {
                    commodity_type: CommodityType::Food,
                    cost_per_unit: 0.5,
                    output_per_tick: 1.0,
                },
                on_planet: OnPlanet {
                    value: planet_earth_id,
                },
            })
            .id();

        let water_factory_id = commands
            .spawn(ManufactoryBundle {
                production: Production {
                    commodity_type: CommodityType::Water,
                    cost_per_unit: 0.3,
                    output_per_tick: 0.75,
                },
                on_planet: OnPlanet {
                    value: planet_earth_id,
                },
            })
            .id();

        let company_id = commands
            .spawn(CompanyBundle::new(
                250.0,
                &vec![food_factory_id, water_factory_id],
                planet_earth_id,
            ))
            .id();

        let mut planet_bundle = PlanetBundle::new(
            "Earth".into(),
            Population {
                consumption: [0.4, 0.3, 0.0],
            },
            SystemCoordinates::new(151_000_000., 10_000_000., 10_000_000.),
        );

        planet_bundle.market.add_members(vec![company_id]);
        planet_bundle.companies.value.extend(vec![company_id]);

        commands.entity(planet_earth_id).insert(planet_bundle);
    }

    // PLANET: Mars
    {
        let planet_mars_id = commands.spawn_empty().id();

        let manufactory_id = commands
            .spawn(ManufactoryBundle {
                production: Production {
                    commodity_type: CommodityType::Clothes,
                    cost_per_unit: 1.5,
                    output_per_tick: 1.0,
                },
                on_planet: OnPlanet {
                    value: planet_mars_id,
                },
            })
            .id();

        let company_id = commands
            .spawn(CompanyBundle::new(
                250.0,
                &vec![manufactory_id],
                planet_mars_id,
            ))
            .id();

        let mut planet_bundle = PlanetBundle::new(
            "Mars".into(),
            Population {
                consumption: [0.1, 0.1, 0.5],
            },
            SystemCoordinates::new(250_000_000., -10_000_000., -10_000_000.),
        );

        planet_bundle.market.add_members(vec![company_id]);
        planet_bundle.companies.value.extend(vec![company_id]);

        commands.entity(planet_mars_id).insert(planet_bundle);
    }

    // TWO LONELY SHIPS
    {
        commands.spawn_batch(vec![
            ShipBundle::new(SystemCoordinates::new(
                200_000_000.,
                5_000_000.,
                -5_000_000.,
            )),
            ShipBundle::new(SystemCoordinates::new(
                200_000_000.,
                -5_000_000.,
                5_000_000.,
            )),
        ]);
    }
}
