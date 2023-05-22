use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, States)]
pub enum GameState {
    #[default]
    LoadingAssets,
    GeneratingAtlases,
    InGame,
}
