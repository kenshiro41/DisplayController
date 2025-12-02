import { Container, Flex, Slider, Text } from '@chakra-ui/react';
import { ReactNode } from 'react';

type Props = {
  title: string;
  value: number;
  onChange: (value: number) => void;
  rightContent?: ReactNode;
  isDisabled?: boolean;
};

export const SliderBar: React.FC<Props> = ({
  title,
  value,
  onChange,
  rightContent,
  isDisabled,
}) => {
  return (
    <Container marginY='2'>
      <Flex justifyContent='space-between'>
        <Text>
          {title}: {value}
        </Text>
        {rightContent && rightContent}
      </Flex>
      <Slider.Root
        value={[value]}
        onValueChange={(e) => onChange(e.value[0])}
        disabled={isDisabled}
        cursor='pointer'
      >
        <Slider.Label />
        <Slider.Control>
          <Slider.Track>
            <Slider.Range />
          </Slider.Track>
          <Slider.Thumb index={0}>
            <Slider.HiddenInput />
          </Slider.Thumb>
        </Slider.Control>
      </Slider.Root>
    </Container>
  );
};
