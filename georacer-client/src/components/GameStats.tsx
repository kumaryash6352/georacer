import React from 'react';

interface Props {
  submitted: number;
  active: number;
  timeLeft: number;
}

const GameStats: React.FC<Props> = ({ submitted, active, timeLeft }) => {
  return (
    <div className="ui-row" style={{ justifyContent: 'center', gap: 8 }}>
      <span className="ui-tag">Submitted: {submitted}/{active}</span>
      <span className="ui-tag">Time Left: {timeLeft}s</span>
    </div>
  );
};

export default GameStats;

