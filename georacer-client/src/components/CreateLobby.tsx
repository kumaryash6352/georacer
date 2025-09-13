import React from 'react';
import { QRCodeCanvas } from 'qrcode.react';
import PlayerName from './PlayerName';

const CreateLobby: React.FC<{ players: string[] }> = ({ players }) => {
  const lobbyId = 'mock-lobby-id'; // Replace with actual lobby ID

  return (
    <div>
      <h1>Create Lobby</h1>
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
      <div>
        <h3>Game Settings</h3>
        <label>
          Points to win:
          <input type="number" defaultValue="5" />
        </label>
        <br />
        <label>
          Players that can score per object:
          <input type="number" defaultValue="1" />
        </label>
      </div>
        <br />
      <button>Start Game</button>
    </div>
  );
};

export default CreateLobby;

