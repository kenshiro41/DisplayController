#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod constants;
mod models;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::set_input_source,
            command::get_displays,
            command::set_brightness,
            command::set_contrast,
            command::set_sharpness,
            command::set_speaker_volume,
            command::set_speaker_mute,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
