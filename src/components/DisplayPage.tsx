import { useState } from 'react';

import { invoke } from '@tauri-apps/api';

import {
  Alert,
  AlertIcon,
  Box,
  Button,
  Flex,
  Heading,
  Select,
  Spacer,
  Text,
} from '@chakra-ui/react';
import { Display } from '../types/display';
import { High, Mute } from './icons/Volume';
import { SliderBar } from './SliderBar';

export const DisplayPage: React.FC<Display> = (props) => {
  const [brightness, setBrightness] = useState(props.brightness);
  const [contrast, setContrast] = useState(props.contrast);
  const [sharpness, setSharpness] = useState(props.sharpness);
  const [speakerVolume, setSpeakerVolume] = useState(props.speaker_volume);
  const [isSpeakerMute, setIsSpeakerMute] = useState(props.speaker_mute === 0);

  const hasErr = (!brightness || !contrast || !sharpness) && !props.brightness;

  const s = Object.entries(sources).map(([key]) => key);

  const handleBrightness = (value: number) => {
    setBrightness(value);
    invoke('set_brightness', {
      id: props.id,
      value,
    });
  };
  const handleContrast = (value: number) => {
    setContrast(value);
    invoke('set_contrast', {
      id: props.id,
      value,
    });
  };
  const handleSharpness = (value: number) => {
    setSharpness(value);
    invoke('set_sharpness', {
      id: props.id,
      value,
    });
  };
  const handleSpeakerVolume = (value: number) => {
    setSpeakerVolume(value);

    invoke('set_speaker_volume', {
      id: props.id,
      value,
    });
  };
  const handleSpeakerMute = () => {
    const value = isSpeakerMute ? speakerVolume : 0;
    if (speakerVolume === 0) {
      setSpeakerVolume(1);
    }
    setIsSpeakerMute(!isSpeakerMute);

    invoke('set_speaker_mute', {
      id: props.id,
      value,
    });
  };

  const handleInputSource: React.ChangeEventHandler<HTMLSelectElement> = async (
    e
  ) => {
    await invoke('set_input_source', {
      id: props.id,
      value: sources[e.target.value],
    });
  };

  return (
    <>
      <Box>
        <Flex flexDir='row' justifyContent='space-between'>
          <Heading>{props.display_name}</Heading>
          <Text justifyContent='center'>
            {props.size.width} x {props.size.height}
          </Text>
        </Flex>

        {hasErr && (
          <Alert status='warning' marginY='4'>
            <AlertIcon />
            ディスプレイ情報を取得できていない可能性があります
          </Alert>
        )}
      </Box>

      <SliderBar title='照度' value={brightness} onChange={handleBrightness} />
      <SliderBar
        title='コントラスト'
        value={contrast}
        onChange={handleContrast}
      />
      <SliderBar
        title='シャープネス'
        value={sharpness}
        onChange={handleSharpness}
      />
      <SliderBar
        title='スピーカー音量'
        value={speakerVolume}
        onChange={handleSpeakerVolume}
        rightContent={
          <Button onClick={handleSpeakerMute}>
            {isSpeakerMute || speakerVolume === 0 ? <Mute /> : <High />}
          </Button>
        }
        isDisabled={isSpeakerMute}
      />

      <Spacer height='4' />

      <Select
        bg={props.active_code ? 'cyan.100' : undefined}
        borderColor={props.active_code ? 'cyan.100' : undefined}
        defaultValue={Object.keys(sources).find(
          (key) => sources[key] === props.active_code
        )}
        onChange={handleInputSource}
      >
        {s.map((x) => (
          <option key={x}>{x}</option>
        ))}
      </Select>
    </>
  );
};

// https://github.com/kfix/ddcctl/blob/main/README.md#input-sources
const sources: { [key: string]: number } = {
  'DisplayPort-1': 15,
  'DisplayPort-2': 16,
  'HDMI-1': 17,
  'HDMI-2': 18,
  'USB-C': 27,
  'VGA-1': 1,
  'VGA-2': 2,
  'DVI-1': 3,
  'DVI-2': 4,
  'Composite video 1': 5,
  'Composite video 2': 6,
  'S-Video-1': 7,
  'S-Video-2': 8,
  'Tuner-1': 9,
  'Tuner-2': 10,
  'Tuner-3': 11,
  'Component video (YPrPb/YCrCb) 1': 12,
  'Component video (YPrPb/YCrCb) 2': 13,
  'Component video (YPrPb/YCrCb) 3': 14,
};
