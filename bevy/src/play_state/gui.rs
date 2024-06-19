use bevy::prelude::*;

use crate::{camera::GameCamera, flow_control::GameState, GAME_HEIGHT, GAME_WIDTH};

use super::{Score, TextConfig};

#[derive(Event)]
pub struct UpdateScore;

#[derive(Component)]
struct RootUiNode;

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

fn spawn(
    mut commands: Commands,
    text_config: Res<TextConfig>,
    camera_query: Query<Entity, With<GameCamera>>,
) {
    let game_camera = camera_query
        .get_single()
        .expect("Unable to get the game camera target");
    let text_style = TextStyle {
        font: text_config.font.clone(),
        font_size: 80.0,
        color: Color::WHITE,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(GAME_WIDTH),
                    height: Val::Px(GAME_HEIGHT),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::FlexStart,
                    //border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                //border_color: Color::ANTIQUE_WHITE.into(),
                ..default()
            },
            TargetCamera(game_camera),
            RootUiNode,
        ))
        .with_children(|ui_node| {
            ui_node.spawn((
                TextBundle {
                    text: Text::from_section("0", text_style.clone())
                        .with_justify(JustifyText::Center),
                    ..default()
                },
                Player::One,
            ));
            ui_node.spawn((
                TextBundle {
                    text: Text::from_section("0", text_style.clone())
                        .with_justify(JustifyText::Center),
                    ..default()
                },
                Player::Two,
            ));
        });
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
