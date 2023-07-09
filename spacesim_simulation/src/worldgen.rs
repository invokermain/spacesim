use crate::common::components::Name;
use bevy::prelude::Commands;

use crate::economy::components::{CommodityConsumer, TargetMarket};
use crate::economy::components::{CommodityProducer, ManufactoryBundle};
use crate::planet::components::{OnPlanet, PlanetBundle};
use crate::ships::bundles::ShipBundle;
use crate::ships::components::SystemCoordinates;

pub fn create_world(mut commands: Commands) {
    // PLANET: Earth
    {
        let planet_earth_id = commands.spawn_empty().id();

        commands.spawn(ManufactoryBundle {
            production: CommodityProducer {
                production: [1.0, 0.0, 0.0],
            },
            on_planet: OnPlanet {
                value: planet_earth_id,
            },
            target_market: TargetMarket {
                value: planet_earth_id,
            },
        });

        commands.spawn(ManufactoryBundle {
            production: CommodityProducer {
                production: [0.0, 0.75, 0.0],
            },
            on_planet: OnPlanet {
                value: planet_earth_id,
            },
            target_market: TargetMarket {
                value: planet_earth_id,
            },
        });

        let planet_bundle = PlanetBundle::new(
            "Earth",
            SystemCoordinates::new(152_000_000., 10_000_000., 10_000_000.),
            CommodityConsumer {
                consumption: [0.4, 0.3, 0.0],
            },
        );

        commands.entity(planet_earth_id).insert(planet_bundle);
    }

    // PLANET: Mars
    {
        let planet_mars_id = commands.spawn_empty().id();

        commands.spawn(ManufactoryBundle {
            production: CommodityProducer {
                production: [0.0, 0.0, 1.0],
            },
            on_planet: OnPlanet {
                value: planet_mars_id,
            },
            target_market: TargetMarket {
                value: planet_mars_id,
            },
        });

        let planet_bundle = PlanetBundle::new(
            "Mars",
            SystemCoordinates::new(250_000_000., -10_000_000., -10_000_000.),
            CommodityConsumer {
                consumption: [0.1, 0.1, 0.5],
            },
        );

        commands.entity(planet_mars_id).insert(planet_bundle);
    }

    // TWO LONELY SHIPS
    {
        commands.spawn_batch(vec![
            ShipBundle::new(
                Name::new("Close To Home"),
                SystemCoordinates::new(200_000_000., 5_000_000., -5_000_000.),
            ),
            // ShipBundle::new(SystemCoordinates::new(
            //     200_000_000.,
            //     -5_000_000.,
            //     5_000_000.,
            // )),
        ]);
    }
}
