use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct GameOver {
    ctx: SharedCtx,
}

impl GameOver {
    pub fn new(ctx: SharedCtx) -> Self {
        GameOver { ctx }
    }
}

impl Screen for GameOver {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        let ctx = self.ctx.borrow();
        let (title, score, subtitle) = match &ctx.last_result {
            Some(r) => (r.title.as_str(), r.score, r.subtitle.as_str()),
            None => ("GAME OVER", 0, ""),
        };
        draw_text(title, 180.0, 180.0, 48.0, theme::ACCENT);
        let score_text = format!("Score: {score}");
        draw_text(&score_text, 230.0, 260.0, 36.0, theme::PACMAN);
        draw_text(subtitle, 150.0, 320.0, 26.0, theme::TEXT);
        draw_text(
            "Press any key for the menu",
            180.0,
            420.0,
            22.0,
            theme::TEXT,
        );
    }

    fn id(&self) -> ScreenId {
        ScreenId::GameOver
    }
}
