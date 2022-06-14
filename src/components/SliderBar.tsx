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
  isDisabled?: boolean
}

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
      <Slider
        aria-label='slider-ex-1'
        color='red'
        defaultValue={value}
        onChange={onChange}
        isDisabled={isDisabled}
      >
        <SliderTrack>
          <SliderFilledTrack bg='cyan.400' />
        </SliderTrack>
        <SliderThumb />
      </Slider>
    </Container>
  )
}
