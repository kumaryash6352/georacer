import React, { useEffect, useRef, useState } from 'react';
import { useWebSocket } from '../contexts/WebSocketContext';
import ObjectDisplay from './ObjectDisplay';
import CameraView from './CameraView';
import config from '../config';

const InGame: React.FC = () => {
  const { socket } = useWebSocket();
  const [target, setTarget] = useState<{ image_b64: string } | null>(null);
  const [countdown, setCountdown] = useState<number | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [cameraReady, setCameraReady] = useState(false);
  const [score, setScore] = useState(0);
  const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
  const cameraRef = useRef<{ takePicture: () => string | null }>(null);

  const showToast = (message: string, type: 'success' | 'error') => {
    setToast({ message, type });
    window.setTimeout(() => setToast(null), 2000);
  };

  // Handle incoming Guess messages and reset a local 20s countdown
  useEffect(() => {
    if (!socket) return;

    const onMessage = (event: MessageEvent) => {
      try {
        const msg = JSON.parse(event.data as string);
        if (msg?.type === 'Guess' && msg?.target) {
          setTarget(msg.target);
          setCountdown(20);
        }
      } catch {
        // ignore malformed messages
      }
    };

    socket.addEventListener('message', onMessage);
    return () => {
      socket.removeEventListener('message', onMessage);
    };
  }, [socket]);

  // Simple local countdown (purely UI) that resets on each Guess
  useEffect(() => {
    if (countdown === null) return;
    const id = window.setInterval(() => {
      setCountdown((c) => (c && c > 0 ? c - 1 : 0));
    }, 1000);
    return () => window.clearInterval(id);
  }, [countdown]);

  const submitGuess = async () => {
    if (!cameraRef.current) return;
    const image_b64 = cameraRef.current.takePicture();
    if (!image_b64) return;
    setSubmitting(true);
    try {
      const res = await fetch(`http://${config.apiUrl}/guess`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ image_b64 })
      });
      if (res.ok) {
        const data = await res.json();
        if (data.correct) {
          setScore((s) => s + 1);
          showToast('Correct!', 'success');
        } else {
          showToast('Not a match, try again.', 'error');
        }
      }
    } catch (e) {
      console.error(e);
      showToast('Submit failed', 'error');
    } finally {
      setSubmitting(false);
    }
  };

  const disabled = submitting || !cameraReady || !target;

  return (
    <div className="ui-container" style={{ paddingTop: 16 }}>
      {/* Toast */}
      {toast && (
        <div
          style={{
            position: 'fixed', top: 16, left: '50%', transform: 'translateX(-50%)', zIndex: 1000,
            padding: '10px 14px', borderRadius: 8, color: '#fff', fontWeight: 600,
            background: toast.type === 'success' ? '#22c55e' : '#ef4444',
            boxShadow: '0 8px 24px rgba(0,0,0,0.18)'
          }}
        >
          {toast.message}
        </div>
      )}

      <div className="ui-stack">
        {/* Score header */}
        <div className="ui-row" style={{ justifyContent: 'center' }}>
          <span className="ui-tag">Score: {score}</span>
        </div>

        <ObjectDisplay target={target} />
        <div className="ui-subtle" style={{ textAlign: 'center' }}>
          {countdown !== null ? `Next target in ~${countdown}s` : 'Waiting for next target...'}
        </div>
        <CameraView ref={cameraRef} onReady={() => setCameraReady(true)} />
        <button className="ui-btn primary" onClick={submitGuess} disabled={disabled}>
          {submitting ? 'Submitting...' : 'Submit Guess'}
        </button>
      </div>
    </div>
  );
};
export default InGame;
