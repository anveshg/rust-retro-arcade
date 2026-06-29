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

        // The game always draws in a fixed 640x480 space. A Camera2D maps that
        // space into a centered, aspect-correct viewport of the real window
        // (which may be any size, especially on web), letterboxing the rest.
        // This works on WebGL1 (no render target / WebGL2 needed).
        clear_background(BLACK);
        let scale = (screen_width() / theme::VIRTUAL_W).min(screen_height() / theme::VIRTUAL_H);
        let vw = theme::VIRTUAL_W * scale;
        let vh = theme::VIRTUAL_H * scale;
        let vx = (screen_width() - vw) / 2.0;
        let vy = (screen_height() - vh) / 2.0;

        let mut cam =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, theme::VIRTUAL_W, theme::VIRTUAL_H));
        // from_display_rect flips Y (intended for render targets); un-flip for
        // direct-to-screen so our top-left-origin draw code renders upright.
        cam.zoom.y = -cam.zoom.y;
        cam.viewport = Some((vx as i32, vy as i32, vw as i32, vh as i32));
        set_camera(&cam);

        draw_rectangle(0.0, 0.0, theme::VIRTUAL_W, theme::VIRTUAL_H, theme::BG);
        app.draw();

        set_default_camera();
        next_frame().await;
    }

    scores::save(&ctx.borrow().scores);
}
