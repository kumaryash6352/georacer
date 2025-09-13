import React from 'react';
import Countdown from './Countdown';
import ObjectDisplay from './ObjectDisplay';
import GameStats from './GameStats';
import CameraView from './CameraView';
import HotColdMeter from './HotColdMeter';

const InGame: React.FC = () => {
  return (
    <div>
      <Countdown />
      <ObjectDisplay />
      <GameStats />
      <CameraView />
      <HotColdMeter />
    </div>
  );
};

export default InGame;
