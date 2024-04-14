use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{flow_control::PlayState, GAME_WIDTH};

use super::PaddleSprite;

pub const PADDLE_WIDTH: f32 = 12.0;
pub const PADDLE_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::Match), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, paddle_sprites: Res<PaddleSprite>) {
    let player_x = GAME_WIDTH / 2.0 - PADDLE_WIDTH * 3.0;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: paddle_sprites.mesh.clone(),
            material: paddle_sprites.material.clone(),
            transform: Transform::from_xyz(player_x * -1.0, 0.0, 10.0),
            ..default()
        },
        Player1,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: paddle_sprites.mesh.clone(),
            material: paddle_sprites.material.clone(),
            transform: Transform::from_xyz(player_x, 0.0, 10.0),
            ..default()
        },
        Player2,
    ));
}
