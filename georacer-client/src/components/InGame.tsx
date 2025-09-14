import React, { useRef, useState, useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { useWebSocket } from '../contexts/WebSocketContext';
import ObjectDisplay from './ObjectDisplay';
import GameStats from './GameStats';
import CameraView from './CameraView';
import HotColdMeter from './HotColdMeter';

const InGame: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { socket, sendMessage } = useWebSocket();
  const { target } = location.state || {};
  const [zoom, setZoom] = useState(1);
  const [feedback, setFeedback] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
  const [guessPending, setGuessPending] = useState(false);

  // Local faux zoom fallback in case UpdateImage is not received
  useEffect(() => {
    const interval = setInterval(() => {
      setZoom((prev) => Math.max(0.1, prev - 0.1));
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, [target]);

  // Handle WebSocket messages for feedback and round navigation
  useEffect(() => {
    if (!socket) return;

    let dismissTimeoutId: number | null = null;

    const handler = (event: MessageEvent) => {
      try {
        const message = JSON.parse((event.data as string) || '{}');
        switch (message.type) {
          case 'GuessResult': {
            if (!guessPending) return; // Only show feedback for our own submission
            if (message.correct) {
              setFeedback({ message: 'Correct! Great job!', type: 'success' });
            } else {
              setFeedback({ message: 'Not quite right, try again!', type: 'error' });
            }
            setGuessPending(false);
            // Auto-dismiss feedback after 3 seconds
            dismissTimeoutId = window.setTimeout(() => setFeedback(null), 3000);
            break;
          }
          case 'UpdateImage': {
            if (typeof message.zoom_level === 'number') {
              setZoom(message.zoom_level);
            }
            break;
          }
          case 'RoundOver': {
            // Navigate to leaderboard within the lobby routes, carrying round scores
            navigate('../leaderboard', { state: { scores: message.scores } });
            break;
          }
          case 'GameOver': {
            // Navigate to leaderboard with final standings
            navigate('../leaderboard', { state: { leaderboard: message.leaderboard, gameOver: true } });
            break;
          }
          default:
            break;
        }
      } catch (e) {
        // ignore malformed messages
      }
    };

    socket.addEventListener('message', handler);
    return () => {
      socket.removeEventListener('message', handler);
      if (dismissTimeoutId) {
        clearTimeout(dismissTimeoutId);
      }
    };
  }, [socket, guessPending, navigate]);

  const cameraRef = useRef<{ takePicture: () => string | null }>(null);

  const handleSubmit = () => {
    if (cameraRef.current) {
      const image_b64 = cameraRef.current.takePicture();
      if (image_b64) {
        setGuessPending(true);
        sendMessage({ type: 'SubmitGuess', image_b64 });
      }
    }
  };

  return (
    <div className="ui-container" style={{ paddingTop: '16px' }}>
      <div className="ui-stack">
        {/* Feedback overlay */}
        {feedback && (
          <div 
            style={{
              position: 'fixed',
              top: '20px',
              left: '50%',
              transform: 'translateX(-50%)',
              zIndex: 1000,
              padding: '12px 20px',
              borderRadius: '8px',
              color: 'white',
              fontWeight: 'bold',
              fontSize: '16px',
              backgroundColor: feedback.type === 'success' ? '#22c55e' : '#ef4444',
              boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
              animation: 'fadeIn 0.3s ease-in-out'
            }}
          >
            {feedback.message}
          </div>
        )}
        
        {/* <div style={{ transform: `scale(${zoom})`, transformOrigin: 'top center' }}> */}
        <ObjectDisplay target={target} />
        {/*</div> */}
        <GameStats />
        <CameraView ref={cameraRef} />
        <HotColdMeter />
        <button onClick={handleSubmit} className="ui-btn primary" disabled={guessPending}>
          {guessPending ? 'Submitting...' : 'Submit'}
        </button>
      </div>
    </div>
  );
};
export default InGame;
