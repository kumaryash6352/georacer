import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { QRCodeCanvas } from 'qrcode.react';
import { useWebSocket } from '../contexts/WebSocketContext';
import PlayerName from './PlayerName';

const Lobby: React.FC = () => {
  const { id: lobbyId } = useParams<{ id: string }>();
  const [players, setPlayers] = useState<string[]>([]);
  const { socket, sendMessage } = useWebSocket();
  const navigate = useNavigate();

  useEffect(() => {
    if (socket) {
      socket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        if (message.type === 'GameState') {
          setPlayers(message.players.map((p: any) => p.name));
      } else if (message.type === 'NewRound') {
        navigate('game', { state: { target: message.target } });
        } else if (message.type === 'Countdown') {
          // TODO: implement countdown UI
          console.log(`Countdown: ${message.duration}`);
        } else {
          console.log(`unknown msg: ${JSON.stringify(message)}`);
        }
      };
    }
  }, [socket, navigate]);

  const startGame = () => {
    sendMessage({ type: 'StartGame' });
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
