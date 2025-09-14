import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useName } from '../contexts/NameContext';
import config from '../config';

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
        body: JSON.stringify({
          points_to_win: pointsToWin,
          scorers_per_target: playersPerObject,
        }),
      });
      if (response.ok) {
        const lobby = await response.json();
        await fetch(`http://${config.apiUrl}/lobby/${lobby}/join`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ name: name.name })
        });
        navigate(`/lobby/${lobby.id}`);
      }
    } catch (error) {
      console.error('Error creating lobby:', error);
    }
  };

  return (
    <div className="ui-container">
      <div className="ui-card">
        <div className="ui-card-body">
          <div className="ui-stack">
            <h1 className="ui-heading">Lobby Settings</h1>
            <div>
              <label>Points to win</label>
              <input className="ui-number" type="number" value={pointsToWin} min={1} onChange={(e) => setPointsToWin(parseInt(e.target.value || '0', 10))} />
            </div>
            <div>
              <label>Players that can score per object</label>
              <input className="ui-number" type="number" value={playersPerObject} min={1} onChange={(e) => setPlayersPerObject(parseInt(e.target.value || '0', 10))} />
            </div>
            <button onClick={handleCreateLobby} className="ui-btn primary">Create Lobby</button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default LobbySettings;

