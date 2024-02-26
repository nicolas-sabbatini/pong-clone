#[allow(clippy::wildcard_imports)]
use constants::*;
use macroquad::prelude::*;
use macroquad_canvas_2d::Canvas2D;
use once_cell::sync::Lazy;
use state_manager::StateManager;

mod constants;
mod goal;
mod play;
mod state_manager;

static FONT: Lazy<Font> = Lazy::new(|| {
    let mut new_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/fff-forward/FFFFORWA.TTF"))
            .unwrap();
    new_font.set_filter(FilterMode::Nearest);
    new_font
});

fn win_config() -> Conf {
    Conf {
        window_title: WINDOW_NAME.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(win_config)]
async fn main() {
    let mut canvas = Canvas2D::new(GAME_WIDTH, GAME_HEIGHT);
    canvas.get_texture().set_filter(FilterMode::Nearest);

    let mut state_manager = StateManager::new();
    state_manager.push(Box::new(play::PlayState::new()));
    state_manager.push(Box::new(goal::GoalState::new(0)));

    loop {
        // TODO: REMOVE THIS
        if is_key_down(KeyCode::Q) {
            miniquad::window::request_quit();
        }

        state_manager.update(&mut canvas);

        canvas.set_camera();
        clear_background(BACKGROUND_COLOR);
        state_manager.draw();
        set_default_camera();
        clear_background(BLACK);
        canvas.draw_to_screen();

        next_frame().await;
    }
}
