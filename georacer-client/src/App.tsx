import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Welcome from './components/Welcome';
import LobbySettings from './components/LobbySettings';
import JoinLobby from './components/JoinLobby';
import Lobby from './components/Lobby';
import './App.css';
import InGame from './components/InGame';
import Leaderboard from './components/Leaderboard';
import AddObject from './components/AddObject';
import { NameProvider } from './contexts/NameContext';
import { LobbyIdProvider } from './contexts/LobbyIdContext';

function App() {
  return (
    <NameProvider>
      <LobbyIdProvider>
        <Router>
          <Routes>
            <Route path="/" element={<Welcome />} />
            <Route path="/settings" element={<LobbySettings />} />
            <Route path="/join" element={<JoinLobby />} />
            <Route path="/lobby/:id" element={<Lobby />} />
            <Route path="/game" element={<InGame />} />
            <Route path="/leaderboard" element={<Leaderboard />} />
            <Route path="/add-object" element={<AddObject />} />
          </Routes>
        </Router>
      </LobbyIdProvider>
    </NameProvider>
  );
}

export default App;
