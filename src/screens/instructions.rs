use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Instructions;

impl Instructions {
    pub fn new() -> Self {
        Instructions
    }
}

impl Screen for Instructions {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        draw_text("INSTRUCTIONS", 200.0, 70.0, 40.0, theme::ACCENT);
        let lines = [
            "Pac-Man: arrow keys or WASD to move.",
            "Eat every pellet to clear the maze.",
            "Avoid the ghosts. Finishing faster scores more.",
            "Choose speed 1-4 at the start.",
            "",
            "Pong: left paddle W/S, right paddle Up/Down.",
            "1 = vs CPU, 2 = two players. First to 7 wins.",
            "",
            "Press any key to return to the menu.",
        ];
        for (i, l) in lines.iter().enumerate() {
            draw_text(l, 60.0, 140.0 + i as f32 * 32.0, 24.0, theme::TEXT);
        }
    }

    fn id(&self) -> ScreenId {
        ScreenId::Instructions
    }
}
