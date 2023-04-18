use crate::economy::{
    components::{CommodityStorage, IsCompany, OnPlanet, OwnedFactories, Population, Wealth},
    market::Market,
};

use super::render_structs::{RenderCompany, RenderPlanet};
use bevy::prelude::{Entity, World};

pub fn get_planet_companies(planet: Entity, world: &mut World) -> Vec<RenderCompany> {
    world
        .query::<(
            Entity,
            &IsCompany,
            &Wealth,
            &CommodityStorage,
            &OwnedFactories,
            &OnPlanet,
        )>()
        .iter(world)
        .filter(|result| result.5.value == planet)
        .map(|result| RenderCompany {
            entity: result.0,
            wealth: result.2.value,
            commodity_storage: result.3.storage,
        })
        .collect()
}

pub fn get_system_planets(world: &mut World) -> Vec<RenderPlanet> {
    world
        .query::<(Entity, &Population, &Market)>()
        .iter(world)
        .map(|result| RenderPlanet {
            entity: result.0,
            population: result.1,
            market: result.2,
        })
        .collect()
}
