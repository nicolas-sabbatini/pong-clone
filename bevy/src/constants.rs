use bevy::prelude::{KeyCode, Vec3};

const UP: Vec3 = Vec3::Y;
const DOWN: Vec3 = Vec3::NEG_Y;

pub const KEY_MAPPING_PLAYER_1: [(KeyCode, Vec3); 2] = [(KeyCode::KeyW, UP), (KeyCode::KeyS, DOWN)];

pub const KEY_MAPPING_PLAYER_2: [(KeyCode, Vec3); 2] =
    [(KeyCode::ArrowUp, UP), (KeyCode::ArrowDown, DOWN)];
