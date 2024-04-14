use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    constants::{KEY_MAPPING_PLAYER_1, KEY_MAPPING_PLAYER_2},
    flow_control::{GameState, PlayState},
    GAME_HEIGHT, GAME_WIDTH,
};

use super::PaddleSprite;

pub const PADDLE_WIDTH: f32 = 12.0;
pub const PADDLE_HEIGHT: f32 = 60.0;
const PADDLE_MOVEMENT: f32 = 250.0;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn_player)
            .add_systems(
                Update,
                ((move_player_1, move_player_2), fix_player_positions)
                    .chain()
                    .run_if(in_state(PlayState::Match)),
            );
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

fn move_player_1(
    mut query: Query<&mut Transform, With<Player1>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query
        .get_single_mut()
        .expect("Unable to get player 1 position on movement");
    for (key, dir) in KEY_MAPPING_PLAYER_1 {
        if keyboard_input.pressed(key) {
            transform.translation += dir * PADDLE_MOVEMENT * time.delta_seconds();
        }
    }
}

fn move_player_2(
    mut query: Query<&mut Transform, With<Player2>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query
        .get_single_mut()
        .expect("Unable to get player 2 position on movement");
    for (key, dir) in KEY_MAPPING_PLAYER_2 {
        if keyboard_input.pressed(key) {
            transform.translation += dir * PADDLE_MOVEMENT * time.delta_seconds();
        }
    }
}

fn fix_player_positions(mut query: Query<&mut Transform, Or<(With<Player1>, With<Player2>)>>) {
    for mut transform in &mut query {
        let paddle_height = PADDLE_HEIGHT / 2.0;
        if transform.translation.y + paddle_height > GAME_HEIGHT / 2.0 {
            transform.translation.y = GAME_HEIGHT / 2.0 - paddle_height;
        }
        if transform.translation.y - paddle_height < GAME_HEIGHT / -2.0 {
            transform.translation.y = GAME_HEIGHT / -2.0 + paddle_height;
        }
    }
}
