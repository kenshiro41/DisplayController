import { useState } from 'react';

import { invoke } from '@tauri-apps/api/core';

import {
  Alert,
  Box,
  Button,
  createListCollection,
  Flex,
  Heading,
  Portal,
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

  const handleInputSource = async (e: { value: string[] }) => {
    const value = Number(e.value[0]);
    await invoke('set_input_source', {
      id: props.id,
      value,
    });
  };

  const selectValue = sourceOptions.items.find(
    (x) => x.value === props.active_code.toString()
  )?.value;

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
          <Alert.Root status='warning'>
            <Alert.Indicator />
            <Alert.Title>
              ディスプレイ情報を取得できていない可能性があります
            </Alert.Title>
          </Alert.Root>
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

      <Select.Root
        collection={sourceOptions}
        value={selectValue ? [selectValue.toString()] : []}
        onValueChange={handleInputSource}
        animationDuration='fast'
        transitionDuration='fast'
      >
        <Select.HiddenSelect />
        <Select.Control>
          <Select.Trigger>
            <Select.ValueText placeholder='Select Input Source' />
          </Select.Trigger>
          <Select.IndicatorGroup>
            <Select.Indicator />
          </Select.IndicatorGroup>
        </Select.Control>
        <Portal>
          <Select.Positioner>
            <Select.Content>
              {sourceOptions.items.map((x) => (
                <Select.Item item={x} key={x.value}>
                  {x.label}
                  <Select.ItemIndicator />
                </Select.Item>
              ))}
            </Select.Content>
          </Select.Positioner>
        </Portal>
      </Select.Root>
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

const sourceOptions = createListCollection({
  items: Object.entries(sources).map(([label, value]) => ({
    label,
    value: value.toString(),
  })),
});
