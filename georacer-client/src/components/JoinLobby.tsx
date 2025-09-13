import React, { useState, useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import config from '../config';

const JoinLobby: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [lobbyId, setLobbyId] = useState('');

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
    <div>
      <h2>Join Lobby</h2>
      <input
        type="text"
        placeholder="Enter Lobby ID"
        value={lobbyId}
        onChange={(e) => setLobbyId(e.target.value)}
      />
      <button onClick={handleJoin}>Join</button>
    </div>
  );
};

export default JoinLobby;

