import React from 'react';

const Leaderboard: React.FC = () => {
  const rows = [
    { name: 'Player 1', score: 100 },
    { name: 'Player 2', score: 80 },
  ];
  return (
    <div className="ui-container">
      <div className="ui-stack lg">
        <h2 className="ui-heading">Leaderboard</h2>
        <div className="ui-card">
          <div className="ui-card-body">
            <div className="ui-stack sm">
              {rows.map((r) => (
                <div key={r.name} className="ui-flex">
                  <span>{r.name}</span>
                  <span className="ui-tag">{r.score}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
        <button className="ui-btn outline">Rejoin Lobby</button>
      </div>
    </div>
  );
};

export default Leaderboard;

