use bevy::prelude::{Resource, States};

#[derive(Resource, Default)]
pub struct InGameState {
    // pub planet_view_state: PlanetViewState,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameViewState {
    #[default]
    System,
    Planet,
    Ship,
    Galaxy,
}
