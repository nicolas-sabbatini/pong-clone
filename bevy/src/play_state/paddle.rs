use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    constants::{KEY_MAPPING_PLAYER_1, KEY_MAPPING_PLAYER_2},
    flow_control::{GameState, PlayState, UpdateStages},
    GAME_HEIGHT, GAME_WIDTH,
};

use super::{
    physics_engine::{HitBox, ReflexTo},
    PaddleSprite,
};

pub const PADDLE_WIDTH: f32 = 12.0;
pub const PADDLE_HEIGHT: f32 = 60.0;
const PADDLE_MOVEMENT: f32 = 250.0;
const PADDLE_SEGMENTS: usize = 5;
const REFLEX_SPEED: f32 = 50.0;

#[derive(Component)]
struct YOffset(f32);

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn)
            .add_systems(OnEnter(PlayState::Serve), restart_y)
            .add_systems(
                Update,
                (
                    (handle_input_player_1, handle_input_player_2).in_set(UpdateStages::Input),
                    fix_y.in_set(UpdateStages::Movement),
                )
                    .run_if(in_state(PlayState::Match)),
            );
    }
}

#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)]
fn spawn(mut commands: Commands, paddle_sprites: Res<PaddleSprite>) {
    let add_segments = |paddle: &mut ChildBuilder, x_pos: f32| {
        for i in 0..PADDLE_SEGMENTS {
            let off_set = i as f32 - 2.0;
            let y_offset = PADDLE_WIDTH * off_set;
            let reflex = if off_set < 0.0 {
                ReflexTo(off_set * REFLEX_SPEED)
            } else if off_set == 0.0 {
                ReflexTo(0.0)
            } else {
                ReflexTo(off_set * REFLEX_SPEED)
            };
            paddle.spawn((
                Transform::from_xyz(x_pos, y_offset, 0.0),
                YOffset(y_offset),
                HitBox {
                    poligon: Rectangle::new(PADDLE_WIDTH, PADDLE_WIDTH),
                },
                reflex,
            ));
        }
    };
    let player_x = GAME_WIDTH / 2.0 - PADDLE_WIDTH * 3.0;

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: paddle_sprites.mesh.clone(),
                material: paddle_sprites.material.clone(),
                transform: Transform::from_xyz(player_x * -1.0, 0.0, 10.0),
                ..default()
            },
            Player1,
        ))
        .with_children(|paddle| add_segments(paddle, player_x * -1.0));
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: paddle_sprites.mesh.clone(),
                material: paddle_sprites.material.clone(),
                transform: Transform::from_xyz(player_x, 0.0, 10.0),
                ..default()
            },
            Player2,
        ))
        .with_children(|paddle| add_segments(paddle, player_x));
}

#[allow(clippy::needless_pass_by_value)]
fn handle_input_player_1(
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

#[allow(clippy::needless_pass_by_value)]
fn handle_input_player_2(
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

fn fix_y(
    mut player_query: Query<(&mut Transform, &Children), Or<(With<Player1>, With<Player2>)>>,
    mut hitbox_query: Query<
        (&mut Transform, &YOffset),
        (With<HitBox>, Without<Player1>, Without<Player2>),
    >,
) {
    for (mut transform, children) in &mut player_query {
        let paddle_height = PADDLE_HEIGHT / 2.0;
        let game_middle_height = GAME_HEIGHT / 2.0;
        if transform.translation.y + paddle_height > game_middle_height {
            transform.translation.y = game_middle_height - paddle_height;
        }
        if transform.translation.y - paddle_height < game_middle_height * -1.0 {
            transform.translation.y = game_middle_height * -1.0 + paddle_height;
        }
        for &child in children {
            if let Ok((mut hitbox_transform, y_offset)) = hitbox_query.get_mut(child) {
                hitbox_transform.translation.y = transform.translation.y + y_offset.0;
            }
        }
    }
}

fn restart_y(mut player_query: Query<&mut Transform, Or<(With<Player1>, With<Player2>)>>) {
    for mut transform in &mut player_query {
        transform.translation.y = 0.0;
    }
}
