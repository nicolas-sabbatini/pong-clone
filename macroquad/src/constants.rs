#![allow(clippy::cast_possible_truncation)]
use macroquad::color::Color;

pub const WINDOW_NAME: &str = "Popng!";

pub const GAME_WIDTH: f32 = 432.0;
pub const GAME_HEIGHT: f32 = 243.0;

pub const WINDOW_WIDTH: i32 = (GAME_WIDTH * 3.0) as i32;
pub const WINDOW_HEIGHT: i32 = (GAME_HEIGHT * 3.0) as i32;

pub const BACKGROUND_COLOR: Color = Color::new(0.0, 0.06, 0.24, 1.0);
pub const PADDLE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const BALL_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const UI_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
