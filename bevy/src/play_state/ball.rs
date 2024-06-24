#![allow(clippy::needless_pass_by_value)]
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    flow_control::{GameState, PlayState, UpdateStages},
    GAME_HEIGHT, GAME_WIDTH,
};

use super::{
    physics_engine::{HitBox, Speed},
    BallSprite, NotifyScore, Player,
};

pub const BALL_WIDTH: f32 = 12.0;
pub const BALL_HEIGHT: f32 = 12.0;
const START_SPEED: f32 = 150.0;

#[derive(Component)]
struct Ball;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn)
            .add_systems(OnEnter(PlayState::Serve), restart)
            .add_systems(OnExit(PlayState::Serve), serve)
            .add_systems(
                Update,
                (fix_y, score)
                    .in_set(UpdateStages::Movement)
                    .run_if(in_state(PlayState::Match)),
            );
    }
}

fn spawn(mut commands: Commands, paddle_sprites: Res<BallSprite>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: paddle_sprites.mesh.clone(),
            material: paddle_sprites.material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        HitBox {
            poligon: Rectangle::new(BALL_WIDTH, BALL_HEIGHT),
        },
        Ball,
        Speed(Vec3::new(150.0, 0.0, 0.0)),
    ));
}

fn restart(mut query: Query<(&mut Transform, &mut Speed), With<Ball>>) {
    let (mut transform, mut speed) = query
        .get_single_mut()
        .expect("Unable to get ball position and speed on movement");
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
    speed.0 = Vec3::ZERO;
}

fn serve(mut query: Query<&mut Speed, With<Ball>>, serve_to: Res<Player>) {
    let mut speed = query
        .get_single_mut()
        .expect("Unable to get ball position and speed on movement");
    match *serve_to {
        Player::One => speed.0.x = START_SPEED,
        Player::Two => speed.0.x = START_SPEED * -1.0,
    }
}

fn fix_y(mut query: Query<(&mut Transform, &mut Speed), With<Ball>>) {
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

fn score(query: Query<&Transform, With<Ball>>, mut event_writer: EventWriter<NotifyScore>) {
    let ball = query.get_single().expect("Unable to get the ball position");
    if ball.translation.x > (GAME_WIDTH / 2.0) + BALL_WIDTH * 2.0 {
        event_writer.send(NotifyScore(Player::Two));
    } else if ball.translation.x < (GAME_WIDTH / -2.0) - BALL_WIDTH * 2.0 {
        event_writer.send(NotifyScore(Player::One));
    }
}
