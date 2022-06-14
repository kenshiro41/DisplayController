use std::fmt::Debug;

use ddc::Ddc;
use ddc_macos::Monitor;

const BRIGHTNESS_CODE: u8 = 0x10;
const SHARPNESS_CODE: u8 = 0x87;
const CONTRAST_CODE: u8 = 0x12;
const INPUT_SOURCE_CODE: u8 = 0x60;
const SPEAKER_VOLUME_CODE: u8 = 0x62;
const SPEAKER_MUTE_CODE: u8 = 0x8d;
const RESET_CODE: u8 = 0x04;

#[derive(Debug, serde::Serialize)]
pub struct Display {
  id: u32,
  display_name: String,
  size: Size,
  active_code: u16,
  brightness: u16,
  contrast: u16,
  sharpness: u16,
  speaker_volume: u16,
  speaker_mute: u16,
}
#[derive(Debug, serde::Serialize)]
struct Size {
  width: u64,
  height: u64,
}

trait Monitors {
  fn get_value(&mut self, code: u8) -> u16;
}

impl Monitors for Monitor {
  fn get_value(&mut self, code: u8) -> u16 {
    match self.get_vcp_feature(code) {
      Ok(v) => v.value(),
      Err(err) => {
        println!("err: {}", err);

        0
      }
    }
  }
}

#[tauri::command]
pub fn get_displays() -> Vec<Display> {
  let mut displays = Vec::new();

  for mut m in Monitor::enumerate().unwrap() {
    let id = m.handle().id;
    let display_name = m.product_name().unwrap();
    let size = Size {
      width: m.handle().pixels_wide(),
      height: m.handle().pixels_high(),
    };
    let active_code = m.get_value(INPUT_SOURCE_CODE);

    let brightness = m.get_value(BRIGHTNESS_CODE);
    let contrast = m.get_value(CONTRAST_CODE);
    let sharpness = m.get_value(SHARPNESS_CODE);
    let speaker_volume = m.get_value(SPEAKER_VOLUME_CODE);
    let speaker_mute = m.get_value(SPEAKER_MUTE_CODE);

    displays.push(Display {
      id: id,
      display_name: display_name,
      size: size,
      active_code: active_code,
      brightness: brightness,
      contrast: contrast,
      sharpness: sharpness,
      speaker_volume: speaker_volume,
      speaker_mute: speaker_mute,
    })
  }

  displays
}

#[tauri::command]
pub fn set_input_source(id: u32, value: u16) {
  set_value(INPUT_SOURCE_CODE, id, value);
}

#[tauri::command]
pub fn set_brightness(id: u32, value: u16) {
  set_value(BRIGHTNESS_CODE, id, value);
}
#[tauri::command]
pub fn set_contrast(id: u32, value: u16) {
  set_value(CONTRAST_CODE, id, value);
}

#[tauri::command]
pub fn set_sharpness(id: u32, value: u16) {
  set_value(SHARPNESS_CODE, id, value);
}
#[tauri::command]
pub fn set_speaker_volume(id: u32, value: u16) {
  set_value(SPEAKER_VOLUME_CODE, id, value);
}
#[tauri::command]
pub fn set_speaker_mute(id: u32, value: u16) {
  set_value(SPEAKER_VOLUME_CODE, id, value);
}

fn set_value(code: u8, id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      let res = m.set_vcp_feature(code, value);
      if res.is_err() {
        println!("err: {:?}", res.err());
      } else {
        println!("code: {}, id: {}, value: {}", code, id, value);
      }
    }
  }
}
