use crate::{
    state_manager::{State, StateResult},
    BACKGROUND_COLOR, FONT, GAME_HEIGHT, GAME_WIDTH, UI_COLOR,
};
use macroquad::{
    color::WHITE,
    input::{is_key_down, KeyCode},
    shapes::draw_rectangle_lines,
    text::{draw_text_ex, measure_text, TextParams},
    texture::draw_texture,
    window::clear_background,
};
use macroquad_canvas_2d::Canvas2D;

const FONT_SIZE: u16 = 16;
const CANVAS_WIDHT: f32 = 170.0;
const CANVAS_HEIGHT: f32 = 45.0;

pub struct GoalState {
    who_score: usize,
    canvas: Option<Canvas2D>,
}

impl GoalState {
    pub fn new(who_score: usize) -> Self {
        GoalState {
            who_score,
            canvas: None,
        }
    }
}

impl State for GoalState {
    fn on_enter(&mut self) {
        let text = format!("Player {} scores", self.who_score + 1);
        let text_size = measure_text(&text, Some(&FONT), FONT_SIZE, 1.0);
        println!("{text_size:#?}");

        let canvas = Canvas2D::new(CANVAS_WIDHT, CANVAS_HEIGHT);
        canvas.set_camera();
        clear_background(BACKGROUND_COLOR);
        draw_rectangle_lines(0.0, 0.0, CANVAS_WIDHT, CANVAS_HEIGHT, 4.0, UI_COLOR);
        let center_text_widht: f32 = (CANVAS_WIDHT - text_size.width) * 0.5 + 1.0;
        let center_text_height: f32 =
            (CANVAS_HEIGHT - text_size.height) * 0.5 + text_size.offset_y + 2.0;
        draw_text_ex(
            &text,
            center_text_widht,
            center_text_height,
            TextParams {
                font: Some(&FONT),
                font_size: FONT_SIZE,
                ..Default::default()
            },
        );
        macroquad::camera::set_default_camera();
        self.canvas = Some(canvas);
    }

    fn update(&mut self, _canvas: &mut macroquad_canvas_2d::Canvas2D) -> StateResult {
        if is_key_down(KeyCode::Space) {
            return StateResult::Pop(1);
        }
        StateResult::None
    }

    fn update_pause(&mut self) {}

    fn draw(&self) {
        let canvas = self.canvas.as_ref().unwrap();
        let offsets_x = (GAME_WIDTH - canvas.width()) * 0.5;
        let offsets_y = (GAME_HEIGHT - canvas.height()) * 0.5;
        draw_texture(canvas.get_texture(), offsets_x, offsets_y, WHITE);
    }

    fn draw_pause(&self) {}
}
