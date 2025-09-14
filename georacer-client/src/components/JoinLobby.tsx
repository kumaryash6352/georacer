import React, { useState, useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import config from '../config';
import { useName } from '../contexts/NameContext';

const JoinLobby: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [lobbyId, setLobbyId] = useState('');
  const name = useName();

  useEffect(() => {
    const lobbyIdFromUrl = searchParams.get('lobby');
    if (lobbyIdFromUrl) {
      setLobbyId(lobbyIdFromUrl);
    }
  }, [searchParams]);

  const handleJoin = async () => {
    if (lobbyId) {
      try {
        const response = await fetch(`http://${config.apiUrl}/lobby/${lobbyId}/join`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ name: name.name })
        });
        if (response.ok) {
          navigate(`/lobby/${lobbyId}`);
        } else {
          console.error('Failed to join lobby');
        }
      } catch (error) {
        console.error('Error joining lobby:', error);
      }
    }
  };

  return (
    <div className="ui-container">
      <div className="ui-card">
        <div className="ui-card-body">
          <div className="ui-stack">
            <h2 className="ui-heading">Join Lobby</h2>
            <input
              className="ui-input"
              placeholder="Enter Lobby ID"
              value={lobbyId}
              onChange={(e) => setLobbyId(e.target.value)}
            />
            <button onClick={handleJoin} className="ui-btn primary">Join</button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default JoinLobby;

