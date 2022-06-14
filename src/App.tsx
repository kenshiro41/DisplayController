import { useEffect, useState } from 'react'

import { invoke } from '@tauri-apps/api/tauri'

import { Button, Center, Container, Spacer, Spinner } from '@chakra-ui/react'
import { DisplayPage } from './components/DisplayPage'
import { Display } from './types/display'

function App() {
  const [displays, setDisplays] = useState<Display[]>([])
  const [isLoading, setIsLoading] = useState(false)

  useEffect(() => {
    const getDisplays = async () => {
      setIsLoading(true)

      const data: Display[] = await invoke('get_displays')
      setDisplays(data)

      setIsLoading(false)
    }

    getDisplays()
  }, [])

  return (
    <Container as='main' marginY='4'>
      <Button
        onClick={() => !isLoading && window.location.reload()}
        disabled={isLoading}
      >
        refresh
      </Button>
      {isLoading && (
        <Center>
          <Spinner size='lg' color='cyan.500' />
        </Center>
      )}
      {displays.map((d) => (
        <div key={d.id}>
          <Spacer height='8' />
          <DisplayPage {...d} />
        </div>
      ))}
    </Container>
  )
}

export default App
