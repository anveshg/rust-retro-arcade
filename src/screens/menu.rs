use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

const ITEMS: [&str; 5] = ["Pac-Man", "Pong", "Instructions", "Credits", "Quit"];

/// Maps a 0-based menu index to its transition.
pub fn menu_target(index: usize) -> Transition {
    match index {
        0 => Transition::Goto(ScreenId::Pacman),
        1 => Transition::Goto(ScreenId::Pong),
        2 => Transition::Goto(ScreenId::Instructions),
        3 => Transition::Goto(ScreenId::Credits),
        _ => Transition::Quit,
    }
}

pub struct Menu {
    ctx: SharedCtx,
    selected: usize,
}

impl Menu {
    pub fn new(ctx: SharedCtx) -> Self {
        Menu { ctx, selected: 0 }
    }
}

impl Screen for Menu {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.up_pressed {
            self.selected = (self.selected + ITEMS.len() - 1) % ITEMS.len();
        }
        if input.down_pressed {
            self.selected = (self.selected + 1) % ITEMS.len();
        }
        if let Some(d) = input.digit {
            let idx = (d as usize).saturating_sub(1);
            if idx < ITEMS.len() {
                return Some(menu_target(idx));
            }
        }
        if input.escape {
            return Some(Transition::Quit);
        }
        if input.enter || input.space {
            self.ctx.borrow().audio.play(crate::audio::Sfx::Select);
            return Some(menu_target(self.selected));
        }
        None
    }

    fn draw(&self) {
        draw_text("RUST RETRO ARCADE", 110.0, 80.0, 44.0, theme::ACCENT);
        for (i, label) in ITEMS.iter().enumerate() {
            let y = 170.0 + i as f32 * 46.0;
            let color = if i == self.selected {
                theme::PACMAN
            } else {
                theme::TEXT
            };
            let prefix = if i == self.selected { "> " } else { "  " };
            let item_text = format!("{}{}. {}", prefix, i + 1, label);
            draw_text(&item_text, 220.0, y, 32.0, color);
        }
        let s = self.ctx.borrow().scores;
        let scores_text = format!(
            "Best Pac-Man: {}    Pong wins: {}",
            s.best_pacman, s.pong_wins
        );
        draw_text(&scores_text, 120.0, 440.0, 22.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Menu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_zero_starts_pacman() {
        assert_eq!(menu_target(0), Transition::Goto(ScreenId::Pacman));
    }

    #[test]
    fn index_one_starts_pong() {
        assert_eq!(menu_target(1), Transition::Goto(ScreenId::Pong));
    }

    #[test]
    fn last_index_quits() {
        assert_eq!(menu_target(4), Transition::Quit);
    }
}
