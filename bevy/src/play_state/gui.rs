#![allow(clippy::needless_pass_by_value)]
use bevy::{prelude::*, utils::tracing::field::debug};

use crate::{camera::GameCamera, flow_control::GameState, GAME_HEIGHT, GAME_WIDTH};

use super::{Score, TextConfig};

#[derive(Event)]
pub struct UpdateScore;

#[derive(Component)]
struct RootUiNode;

#[derive(Component)]
struct UiBlink;

#[derive(Resource, Debug)]
pub struct BlinkStatus {
    pub visible_time: f32,
    pub invisible_time: f32,
    pub is_visible: bool,
    pub timer: Timer,
}

#[derive(Component)]
enum Player {
    One,
    Two,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadAssets), spawn_root_ui)
            .add_systems(
                OnEnter(GameState::RunMainLoop),
                (spawn_score, spawn_ready_mesage).chain(),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::RunMainLoop)),
            )
            .add_systems(Update, blink_text)
            .add_event::<UpdateScore>();
    }
}

fn spawn_root_ui(mut commands: Commands, camera_query: Query<Entity, With<GameCamera>>) {
    let game_camera = camera_query
        .get_single()
        .expect("Unable to get the game camera target");
    commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Px(GAME_WIDTH),
                height: Val::Px(GAME_HEIGHT),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        TargetCamera(game_camera),
        RootUiNode,
    ));
}

fn spawn_score(
    mut commands: Commands,
    text_config: Res<TextConfig>,
    root_ui: Query<Entity, With<RootUiNode>>,
) {
    let text_style = TextStyle {
        font: text_config.font.clone(),
        font_size: 112.0,
        color: Color::WHITE,
    };
    let root_ui = root_ui.get_single().expect("Can't get the root ui");
    let score_text = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(80.0),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
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
        })
        .id();
    commands.entity(root_ui).push_children(&[score_text]);
}

fn spawn_ready_mesage(
    mut commands: Commands,
    text_config: Res<TextConfig>,
    root_ui: Query<Entity, With<RootUiNode>>,
) {
    let root_ui = root_ui.get_single().expect("Can't get the root ui");
    let ready_text = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|ui_node| {
            ui_node.spawn(TextBundle {
                text: Text::from_section(
                    "READY TO\nPLAY?",
                    TextStyle {
                        font: text_config.font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            });
            ui_node.spawn((
                TextBundle {
                    text: Text::from_section(
                        "(PRESS SPACE TO START)",
                        TextStyle {
                            font: text_config.font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    ..default()
                },
                UiBlink,
            ));
        })
        .id();
    commands.entity(root_ui).push_children(&[ready_text]);
}

fn blink_text(
    mut blink_query: Query<&mut Text, With<UiBlink>>,
    time: Res<Time>,
    mut blink_status: ResMut<BlinkStatus>,
) {
    blink_status.timer.tick(time.delta());
    if !blink_status.timer.finished() {
        return;
    }
    blink_status.is_visible = !blink_status.is_visible;
    if blink_status.is_visible {
        blink_status.timer = Timer::from_seconds(blink_status.visible_time, TimerMode::Once);
    } else {
        blink_status.timer = Timer::from_seconds(blink_status.invisible_time, TimerMode::Once);
    }
    for mut text in &mut blink_query {
        if blink_status.is_visible {
            text.sections[0].style.color.set_a(1.0);
        } else {
            text.sections[0].style.color.set_a(0.0);
        }
    }
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
