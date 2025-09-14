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
    <div className="ui-container">
      <div className="ui-stack">
        <h2 className="ui-heading">Lobby {lobbyId}</h2>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: '16px' }}>
          <div className="ui-card">
            <div className="ui-card-body">
              <div className="ui-stack" style={{ alignItems: 'center' }}>
                <strong>Scan to Join</strong>
                <div style={{ background: 'white', padding: 16, borderRadius: 10, boxShadow: 'var(--ui-shadow-sm)' }}>
                  <QRCodeCanvas value={`${window.location.origin}/join?lobby=${lobbyId}`} />
                </div>
              </div>
            </div>
          </div>
          <div className="ui-card">
            <div className="ui-card-body">
              <div className="ui-stack">
                <strong>Players</strong>
                <hr style={{ borderColor: 'var(--ui-border)', opacity: 0.5 }} />
                <div className="ui-stack sm">
                  {players.map((n, i) => (
                    <PlayerName name={n} key={i} />
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <button onClick={startGame} className="ui-btn primary">Start Game</button>
        </div>
      </div>
    </div>
  );
};

export default Lobby;
