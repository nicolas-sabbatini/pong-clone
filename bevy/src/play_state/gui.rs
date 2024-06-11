use bevy::prelude::*;

use crate::flow_control::GameState;

use super::TextConfig;

// Because font config we must offset the text
const TEXT_X_OFF_SET: f32 = 2.0;
const TEXT_X: f32 = 60.0;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn);
    }
}

fn spawn(mut commands: Commands, text_config: Res<TextConfig>) {
    let text_style = TextStyle {
        font: text_config.font.clone(),
        font_size: 80.0,
        color: Color::WHITE,
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section("0", text_style.clone()).with_justify(JustifyText::Center),
        transform: Transform::from_xyz(TEXT_X_OFF_SET + TEXT_X, 170.0, 1.0),
        ..Default::default()
    });
    commands.spawn(Text2dBundle {
        text: Text::from_section("0", text_style.clone()).with_justify(JustifyText::Center),
        transform: Transform::from_xyz((TEXT_X_OFF_SET + TEXT_X) * -1.0, 170.0, 1.0),
        ..Default::default()
    });
}
