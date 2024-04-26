use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    asset_loading::AssetList,
    flow_control::{GameState, PlayState},
};

use self::{
    ball::{BALL_HEIGHT, BALL_WIDTH},
    paddle::{PADDLE_HEIGHT, PADDLE_WIDTH},
};

mod ball;
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

#[derive(Resource)]
struct Score {
    player_1: usize,
    player_2: usize,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::RunMainLoop), start_game)
            .add_systems(OnEnter(PlayState::StartMatch), init_match);

        app.add_plugins((paddle::Plug, ball::Plug, physics_engine::Plug));
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
    let font = asset_server.load("Bubbly_Bold.ttf");
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
}

fn start_game(mut next_state: ResMut<NextState<PlayState>>) {
    next_state.set(PlayState::StartMatch);
}

fn init_match(mut commands: Commands) {
    commands.insert_resource(ServeTo::Right);
    commands.insert_resource(Score {
        player_1: 0,
        player_2: 0,
    });
}
