use macroquad::miniquad::window::request_quit;
use macroquad_canvas_2d::Canvas2D;

#[non_exhaustive]
pub enum StateResult {
    None,
    Change(Box<dyn State>),
    Push(Box<dyn State>),
    Pop(usize),
    Quit,
}

pub trait State {
    fn on_enter(&mut self);
    // We pass the canvas so it can move the camera
    fn update(&mut self, canvas: &mut Canvas2D) -> StateResult;
    fn update_pause(&mut self);
    fn draw(&self);
    fn draw_pause(&self);
}

pub struct StateManager {
    state: Vec<Box<dyn State>>,
}

impl StateManager {
    /// Create a new `StateManager`
    pub fn new() -> Self {
        StateManager { state: Vec::new() }
    }

    /// Push a new state onto the stack
    pub fn push(&mut self, mut state: Box<dyn State>) {
        state.on_enter();
        self.state.push(state);
    }

    /// Pop the current state off the stack
    /// Pop 99 to clear the stack and quit the game maybe?
    pub fn pop(&mut self, num: usize) {
        for _ in 0..num {
            // Call on_exit?
            self.state.pop();
        }
    }

    /// Change the current state to a new state
    pub fn change(&mut self, state: Box<dyn State>) {
        self.pop(1);
        self.push(state);
    }

    /// Update the current state
    pub fn update(&mut self, canvas: &mut Canvas2D) {
        if self.state.is_empty() {
            request_quit();
            return;
        }
        let states = self.state.len() - 1;
        let mut result = StateResult::None;
        for (i, state) in self.state.iter_mut().enumerate() {
            if i == states {
                result = state.update(canvas);
            } else {
                state.update_pause();
            }
        }
        match result {
            StateResult::Change(state) => self.change(state),
            StateResult::Push(state) => self.push(state),
            StateResult::Pop(num) => self.pop(num),
            StateResult::Quit => request_quit(),
            _ => (),
        }
    }

    /// Draw the current state
    pub fn draw(&mut self) {
        if self.state.is_empty() {
            return;
        }
        let states = self.state.len() - 1;
        for (i, state) in self.state.iter().enumerate() {
            if i == states {
                state.draw();
            } else {
                state.draw_pause();
            }
        }
    }
}
