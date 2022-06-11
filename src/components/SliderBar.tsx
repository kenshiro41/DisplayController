import {
  Container,
  Flex,
  Slider,
  SliderFilledTrack,
  SliderThumb,
  SliderTrack,
  Text,
} from '@chakra-ui/react'

type Props = {
  title: string
  value: number
  onChange: (value: number) => void
  rightContent?: JSX.Element
}

export const SliderBar: React.FC<Props> = ({
  title,
  value,
  onChange,
  rightContent,
}) => {
  return (
    <Container marginY='2'>
      <Flex justifyContent='space-between'>
        <Text>
          {title}: {value}
        </Text>
        {rightContent && rightContent}
      </Flex>
      <Slider aria-label='slider-ex-1' defaultValue={value} onChange={onChange}>
        <SliderTrack>
          <SliderFilledTrack />
        </SliderTrack>
        <SliderThumb />
      </Slider>
    </Container>
  )
}
