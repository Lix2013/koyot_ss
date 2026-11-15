//! # The main file
//! mini code for run all logic and settings

// mods
mod manager;


// imports
use macroquad::prelude::*;
use manager::{
    main_systems,
    window::window_config
};

#[macroquad::main(window_config)]
async fn main() {
    // main settings
    show_mouse(false);
    main_systems::start_program().await;
}
