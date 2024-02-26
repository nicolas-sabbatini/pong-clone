use crate::{GAME_HEIGHT, PADDLE_COLOR};
use macroquad::prelude::*;

const PADDLE_SPEED: f32 = 200.0;
pub const PADDEL_WIDTH: f32 = 6.0;
pub const PADDLE_HEIGHT: f32 = 24.0;

#[non_exhaustive]
pub enum ControlScheme {
    WS,
    Arrow,
    IA,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    current_speed: f32,
    pub controls: ControlScheme,
}

impl Paddle {
    pub fn new(x: f32, y: f32, controls: ControlScheme) -> Self {
        Paddle {
            x,
            y,
            current_speed: 0.0,
            controls,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, PADDEL_WIDTH, PADDLE_HEIGHT, PADDLE_COLOR);
    }

    pub fn update(&mut self, dt: f32) {
        self.current_speed = 0.0;
        match self.controls {
            ControlScheme::WS => {
                if is_key_down(KeyCode::W) {
                    self.current_speed -= PADDLE_SPEED * dt;
                }
                if is_key_down(KeyCode::S) {
                    self.current_speed += PADDLE_SPEED * dt;
                }
            }
            ControlScheme::Arrow => {
                if is_key_down(KeyCode::Up) {
                    self.current_speed -= PADDLE_SPEED * dt;
                }
                if is_key_down(KeyCode::Down) {
                    self.current_speed += PADDLE_SPEED * dt;
                }
            }
            _ => (),
        }
        self.y += self.current_speed;
        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y + PADDLE_HEIGHT > GAME_HEIGHT {
            self.y = GAME_HEIGHT - PADDLE_HEIGHT;
        }
    }
}
