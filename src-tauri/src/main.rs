#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  CustomMenuItem, Manager, PhysicalPosition, Position, SystemTray, SystemTrayEvent, SystemTrayMenu,
  TrayIcon,
};

use ddc::Ddc;
use ddc_macos::Monitor;

fn main() {
  let tray = SystemTray::new().with_menu(
    SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("action".to_string(), "Open"))
      .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
  );

  tauri::Builder::default()
    // .menu(menu)
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { position, .. } => {
        let window = app.get_window("main").unwrap();

        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().unwrap();
          window
            .set_position(Position::Physical(PhysicalPosition {
              x: (position.x as i32 - (361 as i32)),
              y: 0,
            }))
            .unwrap();
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
    .invoke_handler(tauri::generate_handler![set_input_source, get_displays])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct Display {
  id: u32,
  display_name: String,
}

#[tauri::command]
fn get_displays() -> Vec<Display> {
  let mut displays = Vec::new();

  for ddc in Monitor::enumerate().unwrap() {
    displays.push(Display {
      id: ddc.handle().id,
      display_name: ddc.product_name().unwrap(),
    })
  }

  displays
}

const INPUT_SOURCE_CODE: u8 = 0x60;

// https://github.com/kfix/ddcctl/blob/main/README.md#input-sources
#[tauri::command]
fn set_input_source(id: u32, value: u16) {
  println!("value is :{:?}", value);

  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(INPUT_SOURCE_CODE, value).unwrap();
    }
  }
}
