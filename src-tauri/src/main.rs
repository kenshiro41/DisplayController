#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod command;
mod constants;
mod menu;
mod models;

use tauri::{
  CustomMenuItem, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent, SystemTrayMenu,
  SystemTrayMenuItem,
};

fn main() {
  let menu = SystemTrayMenu::new()
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("quit", "Quit"));

  let tray = SystemTray::new().with_menu(menu);

  tauri::Builder::default()
    .menu(menu::get_menu())
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { position, .. } => {
        let window = app.get_window("main").unwrap();

        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().unwrap();
          let x = position.x as i32 - (361 as i32);
          let y = position.y as i32 - (840 as i32);
          window
            .set_position(Position::Physical(PhysicalPosition { x: x, y: y }))
            .unwrap();
        }
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handler = app.tray_handle().get_item(&id);

        match id.as_str() {
          "quit" => app.exit(0),
          _ => {}
        }
      }
      _ => {}
    })
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      let tray = app.tray_handle();

      Ok(())
    })
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
