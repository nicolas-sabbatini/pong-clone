use bevy::prelude::{IVec2, KeyCode};

const UP: IVec2 = IVec2::Y;
const DOWN: IVec2 = IVec2::NEG_Y;

pub const DIR_KEY_MAPPING: [(KeyCode, IVec2); 4] = [
    (KeyCode::KeyW, UP),
    (KeyCode::KeyS, DOWN),
    (KeyCode::ArrowUp, UP),
    (KeyCode::ArrowDown, DOWN),
];
