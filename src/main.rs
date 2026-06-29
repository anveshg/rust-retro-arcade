use macroquad::prelude::*;

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
    loop {
        clear_background(Color::new(0.04, 0.04, 0.08, 1.0));
        draw_text("rust-retro-arcade", 180.0, 240.0, 36.0, WHITE);
        next_frame().await;
    }
}
