use bevy::prelude::Color;

// Screen config
pub const WINDOW_TITLE: &str = "pong";

pub const WINDOW_CAMERA_NAME: &str = "windows camera";
pub const WINDOW_CAMERA_CLEAR_COLOR: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 1.0,
};

pub const GAME_WIDTH: f32 = 864.0;
pub const GAME_HEIGHT: f32 = 468.0;
pub const GAME_CAMERA_NAME: &str = "game camera";
pub const GAME_CAMERA_TARGET_NAME: &str = "game camera target";
pub const GAME_CAMERA_CLEAR_COLOR: Color = Color::Rgba {
    red: 0.0,
    green: 0.06,
    blue: 0.24,
    alpha: 1.0,
};
