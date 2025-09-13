import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Welcome from './components/Welcome';
import CreateLobby from './components/CreateLobby';
import JoinLobby from './components/JoinLobby';
import './App.css';

import InGame from './components/InGame';

import Leaderboard from './components/Leaderboard';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Welcome />} />
        <Route path="/create" element={<CreateLobby />} />
        <Route path="/join" element={<JoinLobby />} />
        <Route path="/game" element={<InGame />} />
        <Route path="/leaderboard" element={<Leaderboard />} />
      </Routes>
    </Router>
  );
}

export default App;
