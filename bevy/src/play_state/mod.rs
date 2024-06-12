use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    asset_loading::AssetList,
    constants::KEY_MAPPING_SERVE,
    flow_control::{GameState, PlayState},
};

use self::{
    ball::{BALL_HEIGHT, BALL_WIDTH},
    gui::UpdateScore,
    paddle::{PADDLE_HEIGHT, PADDLE_WIDTH},
};

mod ball;
mod gui;
mod paddle;
mod physics_engine;

#[derive(Resource)]
struct TextConfig {
    font: Handle<Font>,
}

#[derive(Resource)]
struct PaddleSprite {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct BallSprite {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Resource)]
enum ServeTo {
    Right,
    Left,
}

#[derive(Resource, PartialEq, Debug)]
struct Score {
    player_1: usize,
    player_2: usize,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::RunMainLoop), start_game_loop)
            .add_systems(OnEnter(PlayState::InitMatch), init_match)
            .add_systems(
                Update,
                start_match.run_if(in_state(PlayState::InitMatch).and_then(
                    resource_exists_and_equals(Score {
                        player_1: 0,
                        player_2: 0,
                    }),
                )),
            )
            .add_systems(Update, serve.run_if(in_state(PlayState::Serve)));

        app.add_plugins((paddle::Plug, ball::Plug, physics_engine::Plug, gui::Plug));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_list: ResMut<AssetList>,
) {
    let font = asset_server.load("NewHiScore.ttf");
    asset_list.0.push(font.clone().untyped());
    commands.insert_resource(TextConfig { font });

    let paddle_mesh = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let paddle_material = materials.add(Color::rgb(1.0, 1.0, 1.0));
    commands.insert_resource(PaddleSprite {
        mesh: paddle_mesh,
        material: paddle_material,
    });

    let ball_mesh = Mesh2dHandle(meshes.add(Rectangle::new(BALL_WIDTH, BALL_HEIGHT)));
    let ball_material = materials.add(Color::rgb(1.0, 1.0, 1.0));
    commands.insert_resource(BallSprite {
        mesh: ball_mesh,
        material: ball_material,
    });

    commands.insert_resource(ServeTo::Right);
    commands.insert_resource(Score {
        player_1: 0,
        player_2: 0,
    });
}

fn start_game_loop(mut next_state: ResMut<NextState<PlayState>>) {
    info!("Starting game loop");
    next_state.set(PlayState::InitMatch);
}

fn init_match(mut scores: ResMut<Score>, mut update_score_event: EventWriter<UpdateScore>) {
    info!("Creating match");
    scores.player_1 = 0;
    scores.player_2 = 0;
    update_score_event.send(UpdateScore);
}

fn start_match(mut next_state: ResMut<NextState<PlayState>>) {
    info!("Start match Serve state");
    next_state.set(PlayState::Serve);
}

fn serve(keyboard_input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<PlayState>>) {
    if keyboard_input.just_pressed(KEY_MAPPING_SERVE) {
        info!("Serving");
        next_state.set(PlayState::Match);
    }
}
