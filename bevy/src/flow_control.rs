use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    LoadAssets,
    RunMainLoop,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PlayState {
    #[default]
    None,
    Match,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().init_state::<PlayState>();
    }
}
