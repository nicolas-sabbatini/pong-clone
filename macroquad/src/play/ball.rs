use crate::{BALL_COLOR, GAME_HEIGHT};
use macroquad::{math::Vec2, shapes::draw_rectangle};

pub const BALL_SIZE: f32 = 10.0;

pub struct Ball {
    x: f32,
    y: f32,
    vel: Vec2,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Ball {
            x,
            y,
            vel: Vec2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, BALL_SIZE, BALL_SIZE, BALL_COLOR);
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vel.x * dt;
        self.y += self.vel.y * dt;
        if self.y < 0.0 {
            self.y = 0.0;
            self.vel.y = -self.vel.y;
        } else if self.y + BALL_SIZE > GAME_HEIGHT {
            self.y = GAME_HEIGHT - BALL_SIZE;
            self.vel.y = -self.vel.y;
        }
    }
}
