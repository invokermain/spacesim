use crate::economy::components::{CommodityStorage, IsCompany, OnPlanet, OwnedFactories, Wealth};

use super::render_structs::RenderCompany;
use bevy::prelude::{Entity, World};

pub fn get_planet_companies(planet: Entity, world: &mut World) -> Vec<RenderCompany> {
    world
        .query::<(
            &IsCompany,
            &Wealth,
            &CommodityStorage,
            &OwnedFactories,
            &OnPlanet,
        )>()
        .iter(world)
        .filter(|result| result.4.value == planet)
        .map(|result| RenderCompany {
            wealth: result.1.value,
            commodity_storage: result.2.storage,
        })
        .collect()
}
