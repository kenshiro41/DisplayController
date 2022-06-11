import {
  faVolumeHigh,
  faVolumeLow,
  faVolumeMute,
  faVolumeOff,
} from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

export const High = () => <FontAwesomeIcon icon={faVolumeHigh} />

export const Low = () => <FontAwesomeIcon icon={faVolumeLow} />

export const Off = () => <FontAwesomeIcon icon={faVolumeOff} />

export const Mute = () => <FontAwesomeIcon icon={faVolumeMute} />
