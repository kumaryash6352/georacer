import React from 'react';

const JoinLobby: React.FC = () => {
  return (
    <div>
      <h2>Join Lobby</h2>
      <input type="text" placeholder="Enter Lobby ID" />
      <button>Join</button>
      <div>
        {/* Placeholder for QR code scanner */}
        <p>Or scan a QR code</p>
      </div>
    </div>
  );
};

export default JoinLobby;

