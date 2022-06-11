export type Display = {
  id: number
  display_name: string
  size: Size
  active_code: number
  brightness: number
  contrast: number
  sharpness: number
  speaker_volume: number
  speaker_mute: number
}

type Size = {
  width: number
  height: number
}
