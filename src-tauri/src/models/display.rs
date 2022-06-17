#[derive(Debug, serde::Serialize)]
pub struct Display {
  pub id: u32,
  pub display_name: String,
  pub size: Size,
  pub active_code: u16,
  pub brightness: u16,
  pub contrast: u16,
  pub sharpness: u16,
  pub speaker_volume: u16,
  pub speaker_mute: u16,
}
#[derive(Debug, serde::Serialize)]
pub struct Size {
  pub width: u64,
  pub height: u64,
}
