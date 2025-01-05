import { invoke } from '@tauri-apps/api/tauri';

import { Button, Center, Container, Spacer, Spinner } from '@chakra-ui/react';
import { useQuery } from '@tanstack/react-query';
import { DisplayPage } from './components/DisplayPage';
import { Display } from './types/display';

function App() {
  const { data, isLoading, refetch } = useQuery<Display[]>({
    queryKey: ['get-displays'],
    queryFn: () => invoke('get_displays'),
    refetchOnReconnect: true,
    refetchOnWindowFocus: true,
    refetchOnMount: true,
    // 30 minutes
    refetchInterval: 1000 * 60 * 30,
  });

  const displays = data ?? [];

  return (
    <Container as='main' marginY='4'>
      <Button onClick={() => !isLoading && refetch()} disabled={isLoading}>
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
  );
}

export default App;
