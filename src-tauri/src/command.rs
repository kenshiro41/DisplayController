use std::fmt::Debug;

use ddc::Ddc;
use ddc_macos::Monitor;

const BRIGHTNESS_CODE: u8 = 0x10;
const SHARPNESS_CODE: u8 = 0x87;
const CONTRAST_CODE: u8 = 0x12;
const INPUT_SOURCE_CODE: u8 = 0x60;
const SPEAKER_VOLUME: u8 = 0x62;
const SPEAKER_MUTE: u8 = 0x8d;

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
    let active_code = match m.get_vcp_feature(INPUT_SOURCE_CODE) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };

    let brightness = match m.get_vcp_feature(BRIGHTNESS_CODE) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };
    let contrast = match m.get_vcp_feature(CONTRAST_CODE) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };
    let sharpness = match m.get_vcp_feature(SHARPNESS_CODE) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };
    let speaker_volume = match m.get_vcp_feature(SPEAKER_VOLUME) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };
    let speaker_mute = match m.get_vcp_feature(SPEAKER_MUTE) {
      Ok(v) => v.value(),
      Err(err) => 0,
    };

    if cfg!(debug_assertions) {
      let TEST = 0xb7;
      let t = match m.get_vcp_feature(TEST) {
        Ok(v) => {
          println!("test:{:?}", v);

          v.value()
        }
        Err(err) => {
          println!("test:{:?}", err);
          0
        }
      };
    }

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
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(INPUT_SOURCE_CODE, value).unwrap();
    }
  }
}

#[tauri::command]
pub fn set_brightness(id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(BRIGHTNESS_CODE, value).unwrap();
    }
  }
}
#[tauri::command]
pub fn set_contrast(id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(CONTRAST_CODE, value).unwrap();
    }
  }
}
#[tauri::command]
pub fn set_sharpness(id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(SHARPNESS_CODE, value);
    }
  }
}
#[tauri::command]
pub fn set_speaker_volume(id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(SPEAKER_VOLUME, value).unwrap();
    }
  }
}
#[tauri::command]
pub fn set_speaker_mute(id: u32, value: u16) {
  for mut m in Monitor::enumerate().unwrap() {
    let m_id = m.handle().id;
    if m_id == id {
      m.set_vcp_feature(SPEAKER_VOLUME, value).unwrap();
    }
  }
}
