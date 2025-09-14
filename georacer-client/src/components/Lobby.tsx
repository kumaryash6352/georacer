import React from 'react';

const Lobby: React.FC = () => {
  return (
    <div className="ui-container">
      <div className="ui-stack">
        <h2 className="ui-heading">Game</h2>
        <p className="ui-subtle">Lobby has been removed. The server now broadcasts a new target to all connected clients every ~20 seconds.</p>
      </div>
    </div>
  );
};

export default Lobby;
