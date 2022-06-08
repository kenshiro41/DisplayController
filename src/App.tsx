import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import './App.css';

type Display = {
  id: number;
  display_name: string;
};

function App() {
  const [displays, setDisplays] = useState<Display[]>([]);

  useEffect(() => {
    const getDisplays = async () => {
      const data: Display[] = await invoke('get_displays');

      setDisplays(data);
    };

    getDisplays();
  }, []);

  const s = Object.entries(sources).map(([key]) => key);

  return (
    <div className='App'>
      <header className='App-header'>
        {displays.map((d) => (
          <div>
            <h2>{d.display_name}</h2>
            {s.map((x) => (
              <button
                key={x}
                onClick={async () => {
                  console.log(sources[x]);
                  await invoke('set_input_source', {
                    id: d.id,
                    value: sources[x],
                  });
                }}
              >
                {x}: {sources[x]}
              </button>
            ))}
          </div>
        ))}
      </header>
    </div>
  );
}

export default App;

// https://github.com/kfix/ddcctl/blob/main/README.md#input-sources
const sources: { [key: string]: number } = {
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
  'DisplayPort-1': 15,
  'DisplayPort-2': 16,
  'HDMI-1': 17,
  'HDMI-2': 18,
  'USB-C': 27,
};
