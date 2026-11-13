// window.rs

use macroquad::prelude::*;

pub fn window_config() -> Conf {
    Conf {
        window_title: String::from("KOYOT SS"),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        fullscreen: true,
        ..Default::default()
    }
}