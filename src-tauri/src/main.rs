#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod command;
mod menu;
mod utils;

use std::collections::HashMap;

use tauri::{
  CustomMenuItem, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent, SystemTrayMenu,
  SystemTrayMenuItem, SystemTraySubmenu, TrayIcon,
};

fn main() {
  let displays = command::get_displays();

  let menu = SystemTrayMenu::new()
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("quit", "Quit"));

  // let a = SystemTrayMenu::new()
  //   .add_item(CustomMenuItem::new("action".to_string(), "Open"))
  //   .add_submenu(SystemTraySubmenu::new("sub", SystemTrayMenu::new()))
  //   .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

  let sub_menu: SystemTrayMenu = SystemTrayMenu::new();
  // let a = utils::read_json();
  // for (key, value) in a {
  //   sub_menu
  //     .to_owned()
  //     .add_item(CustomMenuItem::new(value.to_string(), key));
  // }

  // for d in displays {
  //   println!("{:?}", d);
  //   menu.to_owned().add_submenu(SystemTraySubmenu::new(
  //     d.display_name,
  //     SystemTrayMenu::new(),
  //   ));
  // }

  let tray = SystemTray::new().with_menu(menu);

  let tray_menu1 = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("toggle", "Toggle"))
    .add_item(CustomMenuItem::new("new", "New window"))
    .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
    .add_item(CustomMenuItem::new("icon_2", "Tray Icon 2"))
    .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
    .add_item(CustomMenuItem::new("exit_app", "Quit"));
  let tray_menu2 = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("toggle", "Toggle"))
    .add_item(CustomMenuItem::new("new", "New window"))
    .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
    .add_item(CustomMenuItem::new("exit_app", "Quit"));

  tauri::Builder::default()
    .menu(menu::get_menu())
    // .system_tray(tray)
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
          println!("{:?}", x);
          window
            .set_position(Position::Physical(PhysicalPosition { x: x, y: 0 }))
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

      window.listen("recording", move |event| {
        if event.payload() == Some("true") {
          tray
            .set_icon(TrayIcon::Raw(include_bytes!("../icons/icon.png").to_vec()))
            .unwrap();
        } else {
          tray
            .set_icon(TrayIcon::Raw(include_bytes!("../icons/icon.png").to_vec()))
            .unwrap();
        }
      });

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
