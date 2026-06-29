use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct PongGame {
    _ctx: SharedCtx,
}
impl PongGame {
    pub fn new(ctx: SharedCtx) -> Self {
        PongGame { _ctx: ctx }
    }
}
impl Screen for PongGame {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }
    fn draw(&self) {
        draw_text("PONG (coming soon)", 170.0, 240.0, 36.0, theme::TEXT);
        draw_text("Esc = menu", 250.0, 300.0, 24.0, theme::TEXT);
    }
    fn id(&self) -> ScreenId {
        ScreenId::Pong
    }
}
