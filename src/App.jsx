import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [isRunning, setIsRunning] = useState(false);

  const startSniffer = async () => {
    await invoke('start_packet_sniffer');
    setIsRunning(true);
  };

  const stopSniffer = async () => {
    await invoke('stop_packet_sniffer');
    setIsRunning(false);
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Packet Sniffer</h1>
        <div>
          <button onClick={startSniffer} disabled={isRunning}>Start Sniffer</button>
          <button onClick={stopSniffer} disabled={!isRunning}>Stop Sniffer</button>
        </div>
      </header>
    </div>
  );
}

export default App;
