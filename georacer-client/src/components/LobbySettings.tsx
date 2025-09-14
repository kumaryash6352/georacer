import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useName } from '../contexts/NameContext';

const LobbySettings: React.FC = () => {
  const navigate = useNavigate();
  const [pointsToWin, setPointsToWin] = useState(5);
  const [playersPerObject, setPlayersPerObject] = useState(1);
  let name = useName();

  const handleCreateLobby = async () => {
    try {
      const response = await fetch('http://localhost:3000/lobby', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify([{
          points_to_win: pointsToWin,
          scorers_per_target: playersPerObject,
        }, { name: name.name }]),
      });
      if (response.ok) {
        const lobby = await response.json();
        navigate(`/lobby/${lobby.id}`);
      }
    } catch (error) {
      console.error('Error creating lobby:', error);
    }
  };

  return (
    <div>
      <h1>Lobby Settings</h1>
      <div>
        <label>
          Points to win:
          <input
            type="number"
            value={pointsToWin}
            onChange={(e) => setPointsToWin(parseInt(e.target.value, 10))}
          />
        </label>
      </div>
      <div>
        <label>
          Players that can score per object:
          <input
            type="number"
            value={playersPerObject}
            onChange={(e) => setPlayersPerObject(parseInt(e.target.value, 10))}
          />
        </label>
      </div>
      <button onClick={handleCreateLobby}>Create Lobby</button>
    </div>
  );
};

export default LobbySettings;

