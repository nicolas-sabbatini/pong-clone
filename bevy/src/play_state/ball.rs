use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    flow_control::{GameState, PlayState},
    GAME_HEIGHT, GAME_WIDTH,
};

use super::BallSprite;

pub const BALL_WIDTH: f32 = 12.0;
pub const BALL_HEIGHT: f32 = 12.0;
const BALL_MOVEMENT: f32 = 250.0;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Speed(Vec3);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn_ball)
            .add_systems(
                Update,
                (move_ball, collide_ball_wall)
                    .chain()
                    .run_if(in_state(PlayState::Match)),
            );
    }
}

fn spawn_ball(mut commands: Commands, paddle_sprites: Res<BallSprite>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: paddle_sprites.mesh.clone(),
            material: paddle_sprites.material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        Ball,
        Speed(Vec3::new(100.0, 400.0, 0.0)),
    ));
}

fn move_ball(mut query: Query<(&mut Transform, &Speed), With<Ball>>, time: Res<Time>) {
    let (mut transform, speed) = query
        .get_single_mut()
        .expect("Unable to get ball position and speed on movement");
    transform.translation += speed.0 * time.delta_seconds();
}

fn collide_ball_wall(mut query: Query<(&mut Transform, &mut Speed), With<Ball>>) {
    let (mut transform, mut speed) = query
        .get_single_mut()
        .expect("Unable to get ball position and speed on movement");
    let ball_height = BALL_HEIGHT / 2.0;
    if transform.translation.y + ball_height > GAME_HEIGHT / 2.0 {
        transform.translation.y = GAME_HEIGHT / 2.0 - ball_height;
        speed.0.y *= -1.0;
    }
    if transform.translation.y - ball_height < GAME_HEIGHT / -2.0 {
        transform.translation.y = GAME_HEIGHT / -2.0 + ball_height;
        speed.0.y *= -1.0;
    }
}