import './App.css';
import { useState } from 'react';
import InGame from './components/InGame';
import AddObject from './components/AddObject';
import { NameProvider } from './contexts/NameContext';
import { WebSocketProvider } from './contexts/WebSocketContext';

function App() {
  const [view, setView] = useState<'play' | 'add'>('play');

  return (
    <NameProvider>
      <div className="ui-container" style={{ paddingTop: 12 }}>
        <div className="ui-row" style={{ justifyContent: 'center', gap: 8, marginBottom: 12 }}>
          <button className={`ui-btn ${view === 'play' ? 'primary' : 'outline'}`} onClick={() => setView('play')}>Play</button>
          <button className={`ui-btn ${view === 'add' ? 'primary' : 'outline'}`} onClick={() => setView('add')}>Add Object</button>
        </div>
        {view === 'play' ? (
          <WebSocketProvider>
            <InGame />
          </WebSocketProvider>
        ) : (
          <AddObject />
        )}
      </div>
    </NameProvider>
  );
}

export default App;
