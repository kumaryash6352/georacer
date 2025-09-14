import React, { useRef, useState, useEffect } from 'react';
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
  const [zoom, setZoom] = useState(1);

  useEffect(() => {
    const interval = setInterval(() => {
      setZoom((prev) => Math.max(0.1, prev - 0.1));
    }, 1000);

    const timeout = setTimeout(() => {
      sendMessage({ type: 'StartGame' });
    }, 10000);

    return () => {
      clearInterval(interval);
      clearTimeout(timeout);
    };
  }, [target, sendMessage]);
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
<div style={{ transform: `scale(${zoom})` }}>
        <ObjectDisplay target={target} />
      </div>
      <GameStats />
      <CameraView ref={cameraRef} />
      <HotColdMeter />
      <button onClick={handleSubmit}>Submit</button>
    </div>
  );
};
export default InGame;
