use bevy::{
    math::bounding::{AabbCast2d, Bounded2d, RayCast2d},
    prelude::*,
};

use crate::flow_control::{PlayState, UpdateStages};

const REFLEX_MUL: f32 = -1.17;

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
        app.add_systems(
            Update,
            check_collisions
                .in_set(UpdateStages::Collitions)
                .run_if(in_state(PlayState::Match)),
        );
    }
}

fn calculate_ray(origin: Vec3, size: Vec3) -> RayCast2d {
    let direction = Direction2d::new_unchecked(size.xy().normalize());
    RayCast2d::from_ray(
        Ray2d {
            origin: origin.xy(),
            direction,
        },
        size.length(),
    )
}

#[allow(clippy::needless_pass_by_value)]
fn check_collisions(
    paddle_hitbox: Query<(&HitBox, &Transform, &ReflexTo), Without<Speed>>,
    mut ball_hitbox: Query<(&HitBox, &mut Transform, &mut Speed), Without<ReflexTo>>,
    time: Res<Time>,
) {
    let (ball_hitbox, mut ball_transform, mut speed) = ball_hitbox
        .get_single_mut()
        .expect("Unable to get ball on check collisions");

    let ray = calculate_ray(ball_transform.translation, speed.0 * time.delta_seconds());
    let aabb_cast = AabbCast2d {
        aabb: ball_hitbox.poligon.aabb_2d(
            Vec2::ZERO,
            ball_transform.rotation.to_euler(EulerRot::YXZ).2,
        ),
        ray,
    };
    for (paddle_hitbox, paddle_transform, reflect) in &paddle_hitbox {
        let paddle_aabb = paddle_hitbox.poligon.aabb_2d(
            paddle_transform.translation.xy(),
            paddle_transform.rotation.to_euler(EulerRot::YXZ).2,
        );
        if let Some(collision_distance) = aabb_cast.aabb_collision_at(paddle_aabb) {
            speed.0.x *= REFLEX_MUL;
            speed.0.y += reflect.0;
            let new_ball_pos =
                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * collision_distance;
            ball_transform.translation.x = new_ball_pos.x;
            ball_transform.translation.y = new_ball_pos.y;
            break;
        }
    }
    ball_transform.translation += speed.0 * time.delta_seconds();
}
