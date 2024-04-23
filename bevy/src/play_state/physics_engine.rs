use bevy::{
    math::bounding::{Bounded2d, IntersectsVolume},
    prelude::*,
};

use crate::flow_control::{PlayState, UpdateStages};

use super::ball::BALL_HEIGHT;

#[derive(Component)]
pub struct HitBox {
    pub poligon: Rectangle,
}

#[derive(Component, Debug)]
pub struct ReflexTo(pub f32);

#[derive(Component)]
pub struct Speed(pub Vec3);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_coliders.in_set(UpdateStages::Debug))
            .add_systems(
                Update,
                check_colitions
                    .in_set(UpdateStages::Colitions)
                    .run_if(in_state(PlayState::Match)),
            );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn draw_coliders(mut gizmos: Gizmos, query: Query<(&HitBox, &Transform)>) {
    for (hit_box, transform) in query.iter() {
        gizmos.primitive_2d(
            hit_box.poligon,
            transform.translation.xy(),
            transform.rotation.to_euler(EulerRot::YXZ).2,
            Color::RED,
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn check_colitions(
    paddle_hitbox: Query<(&HitBox, &Transform, &ReflexTo), Without<Speed>>,
    mut ball_hitbox: Query<(&HitBox, &mut Transform, &mut Speed), Without<ReflexTo>>,
) {
    let (ball_hitbox, mut ball_transform, mut speed) = ball_hitbox
        .get_single_mut()
        .expect("Unable to get ball on check colitions");
    let ball_aabb = ball_hitbox.poligon.aabb_2d(
        ball_transform.translation.xy(),
        ball_transform.rotation.to_euler(EulerRot::YXZ).2,
    );
    for (paddle_hitbox, paddle_transform, reflect) in &paddle_hitbox {
        let paddle_aabb = paddle_hitbox.poligon.aabb_2d(
            paddle_transform.translation.xy(),
            paddle_transform.rotation.to_euler(EulerRot::YXZ).2,
        );
        if paddle_aabb.intersects(&ball_aabb) {
            if paddle_transform.translation.x > ball_transform.translation.x {
                ball_transform.translation.x = paddle_transform.translation.x - BALL_HEIGHT;
            } else {
                ball_transform.translation.x = paddle_transform.translation.x + BALL_HEIGHT;
            }
            speed.0.x *= -1.01;
            speed.0.y += reflect.0;
            break;
        }
    }
}
