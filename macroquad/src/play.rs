use crate::{
    state_manager::{State, StateResult},
    FONT, GAME_HEIGHT, GAME_WIDTH,
};
use ball::{Ball, BALL_SIZE};
use macroquad::text::{draw_text_ex, measure_text, TextDimensions, TextParams};
use once_cell::sync::Lazy;
use paddle::{ControlScheme, Paddle, PADDEL_WIDTH, PADDLE_HEIGHT};

mod ball;
mod paddle;

const RIGHT_PADDLE_X: f32 = 15.0;
const LEFT_PADDLE_X: f32 = GAME_WIDTH - RIGHT_PADDLE_X - PADDEL_WIDTH;
const PADDLE_Y: f32 = (GAME_HEIGHT - PADDLE_HEIGHT) / 2.0;

static TEXT_SIZE: Lazy<TextDimensions> = Lazy::new(|| measure_text("0", Some(&FONT), 20, 1.0));

pub struct PlayState {
    players: [Paddle; 2],
    ball: Ball,
    score: [u32; 2],
}

impl PlayState {
    pub fn new() -> Self {
        PlayState {
            players: [
                Paddle::new(RIGHT_PADDLE_X, PADDLE_Y, ControlScheme::WS),
                Paddle::new(LEFT_PADDLE_X, PADDLE_Y, ControlScheme::Arrow),
            ],
            ball: Ball::new(
                (GAME_WIDTH - BALL_SIZE) / 2.0,
                (GAME_HEIGHT - BALL_SIZE) / 2.0,
            ),
            score: [0, 0],
        }
    }
}

impl State for PlayState {
    fn on_enter(&mut self) {}

    fn update(&mut self, _canvas: &mut macroquad_canvas_2d::Canvas2D) -> StateResult {
        let dt = macroquad::time::get_frame_time();
        for player in &mut self.players {
            player.update(dt);
        }
        self.ball.update(dt);
        StateResult::None
    }

    fn update_pause(&mut self) {}

    fn draw(&self) {
        draw(self);
    }

    fn draw_pause(&self) {
        draw(self);
    }
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn draw(state: &PlayState) {
    // Draw game
    for player in &state.players {
        player.draw();
    }
    state.ball.draw();

    // Draw ui
    for (i, score) in state.score.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let x = GAME_WIDTH * 0.5 - TEXT_SIZE.width - 50.0 + ((TEXT_SIZE.width + 100.0) * i as f32);
        #[allow(clippy::cast_precision_loss)]
        draw_text_ex(
            &score.to_string(),
            x,
            50.0,
            TextParams {
                font_size: 20,
                font: Some(&FONT),
                ..Default::default()
            },
        );
    }
}
