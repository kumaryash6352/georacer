import React, { useRef } from 'react';
import { useLocation } from 'react-router-dom';
import { useWebSocket } from '../contexts/WebSocketContext';
import Countdown from './Countdown';
import ObjectDisplay from './ObjectDisplay';
import GameStats from './GameStats';
import CameraView from './CameraView';
import HotColdMeter from './HotColdMeter';

const InGame: React.FC = () => {
  const location = useLocation();
  const { sendMessage } = useWebSocket();
  const { target } = location.state || {};
  const cameraRef = useRef<{ takePicture: () => string | null }>(null);

  const handleSubmit = () => {
    if (cameraRef.current) {
      const image_b64 = cameraRef.current.takePicture();
      if (image_b64) {
        sendMessage({ type: 'SubmitGuess', image_b64 });
      }
    }
  };

  return (
    <div style={{ paddingTop: '25vh' }}>
      <ObjectDisplay target={target} />
      <GameStats />
      <CameraView ref={cameraRef} />
      <HotColdMeter />
      <button onClick={handleSubmit}>Submit</button>
    </div>
  );
};
export default InGame;
