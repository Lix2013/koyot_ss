// mods
mod manager;

// imports
use macroquad::prelude::*;
use manager::{
    prelude,
    window::window_config
};


#[macroquad::main(window_config)]
async fn main() {
    // main settings
    show_mouse(false);
    prelude::start_program().await;
}
