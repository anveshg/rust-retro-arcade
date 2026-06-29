use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Credits;

impl Credits {
    pub fn new() -> Self {
        Credits
    }
}

impl Screen for Credits {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        draw_text("CREDITS", 220.0, 120.0, 48.0, theme::ACCENT);
        draw_text("ANVESH", 240.0, 230.0, 56.0, theme::PACMAN);
        draw_text(
            "Rust rewrite of an old DOS C++ project",
            110.0,
            320.0,
            24.0,
            theme::TEXT,
        );
        draw_text("Press any key to return", 200.0, 420.0, 22.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Credits
    }
}
