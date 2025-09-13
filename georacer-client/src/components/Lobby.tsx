import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { QRCodeCanvas } from 'qrcode.react';
import PlayerName from './PlayerName';
import config from '../config';

const Lobby: React.FC = () => {
  const { id: lobbyId } = useParams<{ id: string }>();
  const [players, setPlayers] = useState<string[]>([]);
  const [socket, setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(`ws://${config.apiUrl}/ws/${lobbyId}`);
    setSocket(ws);

    ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      if (message.type === 'player_list') {
        setPlayers(message.players);
      }
    };

    return () => {
      ws.close();
    };
  }, [lobbyId]);

  const startGame = () => {
    if (socket) {
      socket.send(JSON.stringify({ type: 'start_game' }));
    }
  };

  return (
    <div>
      <h1>Lobby {lobbyId}</h1>
      <div>
        <h3>Scan to Join</h3>
        <QRCodeCanvas value={`${window.location.origin}/join?lobby=${lobbyId}`} />
      </div>
      <div>
        <h3>Players</h3>
        <ul>
          {players.map((n, i) => <PlayerName name={n} key={i} />)}
        </ul>
      </div>
      <br />
      <button onClick={startGame}>Start Game</button>
    </div>
  );
};

export default Lobby;
