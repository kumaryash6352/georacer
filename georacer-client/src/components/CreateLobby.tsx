import React from 'react';
import { QRCodeCanvas } from 'qrcode.react';

const CreateLobby: React.FC = () => {
  const lobbyId = 'mock-lobby-id'; // Replace with actual lobby ID

  return (
    <div>
      <h2>Create Lobby</h2>
      <div>
        <h3>Scan to Join</h3>
        <QRCodeCanvas value={`${window.location.origin}/join?lobby=${lobbyId}`} />
      </div>
      <div>
        <h3>Players</h3>
        <ul>
          {/* Placeholder for player list */}
          <li>Player 1</li>
          <li>Player 2</li>
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
      <button>Start Game</button>
    </div>
  );
};

export default CreateLobby;

