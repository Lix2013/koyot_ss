// prelude.rs

//! # App logic
//! ---
//! the app logic is simple
//! the program starts in start_program function
//! load the main textures and vars
//! 
//! he start in the first loop for 5s write [ KOYOT SS ] in screen
//! the 2nd loop is the main logic : windows, inputs ...
//! and the 3th loop he is a exit splash write in screen [ KOYOT SS ] for 5s

// imports
use macroquad::prelude::*;
use crate::manager::log;

const MOUSE_BYTES: &[u8] = include_bytes!("../../assets/mouse.png");

pub async fn start_program() {
    log::log(log::LogStates::Info, "Adding vars to RAM ...");
    // main vars
    let mut open_win = true;
    let welcome_text = "[ KOYOT SS ]";
    let welcome_text_data = measure_text(welcome_text, None, 50, 1.0);
    let mut open_welcome_win = true;
    let image = macroquad::texture::Image::from_file_with_format(MOUSE_BYTES, None).unwrap();
    let texture = macroquad::texture::Texture2D::from_image(&image);
    let mut should_exit = false;
    log::log(log::LogStates::Success, "Vars Added !");

    // boot
    log::log(log::LogStates::Info, "Booting ...");
    let start_boot_time = get_time();
    loop {
        clear_background(BLACK);
    
        draw_text(welcome_text, screen_width() / 2.0 - welcome_text_data.width / 2.0, screen_height() / 2.0 - welcome_text_data.height / 2.0, 50.0, WHITE);

        if get_time() - start_boot_time >= 5.0 {
            break;
        }

        next_frame().await
    }
    log::log(log::LogStates::Success, "Booting finished !");

    // main loop
    log::log(log::LogStates::Info, "Starting main loop");
    loop {
        clear_background(BLACK);

        // if blocks
        if is_key_pressed(KeyCode::S) {
            if open_win {
                println!("Removed all windows");
            } else if open_win != true {
                println!("Added all windows")
            }
            open_win = !open_win;
        }
        if is_key_pressed(KeyCode::W) {
            if open_welcome_win {
                println!("Removed welcome window");
            } else if open_welcome_win != true {
                println!("Added welcome window")
            }
            open_welcome_win = !open_welcome_win;
        }
        if is_key_pressed(KeyCode::Escape) {
            should_exit = true;
        }
        if should_exit {
            break;
        }

        // ui
        egui_macroquad::ui(|ctx_ui| {
            if open_win {
                egui_macroquad::egui::Window::new("Welcome to koyot ss")
                    .open(&mut open_welcome_win)
                    .max_width(300.0)
                    .max_height(300.0)
                    .show(ctx_ui, |ui| {
                        ui.heading("welcome to koyot ss");
                        ui.separator();
                        ui.label("this is a mini Environment");
                    });
                egui_macroquad::egui::Window::new("Dashboard")
                    .show(ctx_ui, |ui| {
                        ui.heading("Dashboard");
                        let exit_button = ui.button("Exit");

                        if exit_button.clicked() {
                            should_exit = true;
                        }
                    });
            }
        });

        egui_macroquad::draw();

        // draw mouse
        show_mouse(false);
        let (mut mouse_pos_x, mut mouse_pos_y) = mouse_position();

        mouse_pos_x = mouse_pos_x.clamp(0.0, screen_width() - texture.width());
        mouse_pos_y = mouse_pos_y.clamp(0.0, screen_height() - texture.height());
        draw_texture(&texture, mouse_pos_x, mouse_pos_y, WHITE);

        next_frame().await;
    }
    // exit splash
    let start_sp_time = get_time();
    loop {
        clear_background(BLACK);
    
        draw_text(welcome_text, screen_width() / 2.0 - welcome_text_data.width / 2.0, screen_height() / 2.0 - welcome_text_data.height / 2.0, 50.0, WHITE);

        if get_time() - start_sp_time >= 5.0 {
            break;
        }

        next_frame().await
    }
    log::log(log::LogStates::Info, "Exiting ...");
}
