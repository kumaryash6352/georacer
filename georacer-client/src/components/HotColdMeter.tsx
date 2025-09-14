import React from 'react';

const HotColdMeter: React.FC = () => {
  const value = 70; // placeholder
  return (
    <div className="ui-stack sm">
      <span className="ui-subtle" style={{ fontSize: '0.9em' }}>Hot/Cold</span>
      <div className="ui-progress" style={{ ['--value' as any]: `${value}%` }}>
        <span></span>
      </div>
    </div>
  );
};

export default HotColdMeter;

