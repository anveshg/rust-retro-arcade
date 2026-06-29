pub mod maze;
pub mod player;

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct PacmanGame {
    _ctx: SharedCtx,
}
impl PacmanGame {
    pub fn new(ctx: SharedCtx) -> Self {
        PacmanGame { _ctx: ctx }
    }
}
impl Screen for PacmanGame {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }
    fn draw(&self) {
        draw_text("PAC-MAN (coming soon)", 150.0, 240.0, 36.0, theme::TEXT);
        draw_text("Esc = menu", 250.0, 300.0, 24.0, theme::TEXT);
    }
    fn id(&self) -> ScreenId {
        ScreenId::Pacman
    }
}
