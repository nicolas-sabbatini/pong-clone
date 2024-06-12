use bevy::prelude::*;

use crate::flow_control::GameState;

use super::{Score, TextConfig};

// Because font config we must offset the text
const TEXT_X_OFF_SET: f32 = 2.0;
const TEXT_X: f32 = 60.0;

#[derive(Event)]
pub struct UpdateScore;

#[derive(Component)]
enum Player {
    One,
    Two,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RunMainLoop), spawn)
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::RunMainLoop)),
            )
            .add_event::<UpdateScore>();
    }
}

fn spawn(mut commands: Commands, text_config: Res<TextConfig>) {
    let text_style = TextStyle {
        font: text_config.font.clone(),
        font_size: 80.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", text_style.clone()).with_justify(JustifyText::Center),
            transform: Transform::from_xyz((TEXT_X_OFF_SET + TEXT_X) * -1.0, 170.0, 1.0),
            ..Default::default()
        },
        Player::One,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", text_style.clone()).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(TEXT_X_OFF_SET + TEXT_X, 170.0, 1.0),
            ..Default::default()
        },
        Player::Two,
    ));
}

fn update_score(
    mut query: Query<(&mut Text, &Player)>,
    mut update_score_event: EventReader<UpdateScore>,
    scores: Res<Score>,
) {
    if update_score_event.is_empty() {
        return;
    }
    info!("Updating scores: {scores:?}");
    update_score_event.clear();
    for (mut text, player) in &mut query {
        match player {
            Player::One => text.sections[0].value = scores.player_1.to_string(),
            Player::Two => text.sections[0].value = scores.player_2.to_string(),
        }
    }
}
