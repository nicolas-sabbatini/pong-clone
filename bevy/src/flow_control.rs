use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateStages {
    Input,
    Movement,
    Collitions,
}

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
    InitMatch,
    Serve,
    Match,
    Score,
    GameOver,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<PlayState>()
            .configure_sets(
                Update,
                (
                    UpdateStages::Input,
                    UpdateStages::Movement.after(UpdateStages::Input),
                    UpdateStages::Collitions.after(UpdateStages::Movement),
                )
                    .run_if(in_state(GameState::RunMainLoop)),
            );
    }
}
