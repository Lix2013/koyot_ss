// window.rs

//! # The window config :
//! ---
//! title : KOYOT SS
//! full screen : true

use macroquad::prelude::*;

pub fn window_config() -> Conf {
    Conf {
        window_title: String::from("KOYOT SS"),
        fullscreen: true,
        ..Default::default()
    }
}