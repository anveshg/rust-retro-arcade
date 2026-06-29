use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod app;
mod audio;
mod input;
mod pacman;
mod pong;
mod scores;
mod screens;
mod theme;

use app::{App, Screen, ScreenId};
use input::Input;

pub struct GameResult {
    pub title: String,
    pub score: u32,
    pub subtitle: String,
}

pub struct Ctx {
    pub audio: audio::Audio,
    pub scores: scores::HighScores,
    pub last_result: Option<GameResult>,
}

pub type SharedCtx = Rc<RefCell<Ctx>>;

fn make_screen(id: ScreenId, ctx: SharedCtx) -> Box<dyn Screen> {
    match id {
        ScreenId::Menu => Box::new(screens::Menu::new(ctx)),
        ScreenId::Instructions => Box::new(screens::Instructions::new()),
        ScreenId::Credits => Box::new(screens::Credits::new()),
        ScreenId::Pong => Box::new(pong::PongGame::new(ctx)),
        ScreenId::Pacman => Box::new(pacman::PacmanGame::new(ctx)),
        ScreenId::GameOver => Box::new(screens::GameOver::new(ctx)),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "rust-retro-arcade".to_owned(),
        window_width: 640,
        window_height: 480,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let audio = audio::Audio::load().await;
    let scores = scores::load();
    let ctx: SharedCtx = Rc::new(RefCell::new(Ctx {
        audio,
        scores,
        last_result: None,
    }));

    let factory_ctx = ctx.clone();
    let mut app = App::new(
        move |id| make_screen(id, factory_ctx.clone()),
        ScreenId::Menu,
    );

    loop {
        let input = Input::poll();
        let dt = get_frame_time();
        app.update(&input, dt);
        if app.quit {
            break;
        }
        clear_background(theme::BG);
        app.draw();
        next_frame().await;
    }

    scores::save(&ctx.borrow().scores);
}
