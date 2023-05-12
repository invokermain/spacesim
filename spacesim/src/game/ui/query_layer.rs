use spacesim_simulation::{
    common::marker_components::{IsCompany, IsPlanet},
    economy::{
        components::{CommodityStorage, OnPlanet, OwnedFactories, Population, Wealth},
        market::Market,
    },
};

use super::render_structs::{RenderCompany, RenderPlanet, RenderSystemInfo};
use bevy::prelude::{Entity, With, World};

pub fn get_planet_companies(planet: Entity, world: &mut World) -> Vec<RenderCompany> {
    world
        .query::<(
            (
                Entity,
                &Wealth,
                &CommodityStorage,
                &OwnedFactories,
                &OnPlanet,
            ),
            With<IsCompany>,
        )>()
        .iter(world)
        .map(|result| result.0)
        .filter(|result| result.4.value == planet)
        .map(|result| RenderCompany {
            entity: result.0,
            wealth: result.1.value,
            commodity_storage: result.2.storage,
        })
        .collect()
}

pub fn get_system_info(world: &mut World) -> RenderSystemInfo {
    RenderSystemInfo {
        planets: world
            .query::<(Entity, With<IsPlanet>)>()
            .iter(world)
            .map(|res| res.0)
            .collect(),
    }
}

pub fn get_planet(world: &mut World, planet_id: Entity) -> RenderPlanet {
    let (market, population) = world
        .query::<((&Market, &Population), With<IsPlanet>)>()
        .get(world, planet_id)
        .unwrap()
        .0;

    RenderPlanet {
        entity: planet_id,
        market: market.clone(),
        population: *population,
    }
}
