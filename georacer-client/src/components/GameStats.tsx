import React from 'react';

const GameStats: React.FC = () => {
  return (
    <div className="ui-row" style={{ justifyContent: 'center' }}>
      <span className="ui-tag">Players Found: 0</span>
      <span className="ui-tag">Time: 0s</span>
    </div>
  );
};

export default GameStats;

