import React, { useEffect, useMemo, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { useWebSocket } from '../contexts/WebSocketContext';

interface ScoreRow { name: string; score: number }

const Leaderboard: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { socket } = useWebSocket();

  // Prefer scores passed via navigation state from RoundOver
  const initialScores: Record<string, number> | undefined = (location.state as any)?.scores;
  const gameOverLeaderboard: [ { name: string }, number ][] | undefined = (location.state as any)?.leaderboard;
  const isGameOver = Boolean((location.state as any)?.gameOver);

  const [targetName, setTargetName] = useState<string | null>(null);

  const rows: ScoreRow[] = useMemo(() => {
    if (gameOverLeaderboard && Array.isArray(gameOverLeaderboard)) {
      // leaderboard is Vec<(Player, f32)> -> serialized as [[{"name":...}, score], ...]
      return gameOverLeaderboard.map(([player, score]: any) => ({ name: player?.name ?? 'Unknown', score }));
    }
    if (initialScores) {
      return Object.entries(initialScores)
        .map(([name, score]) => ({ name, score: Number(score) }))
        .sort((a, b) => b.score - a.score);
    }
    return [];
  }, [initialScores, gameOverLeaderboard]);

  // While on leaderboard, listen for the next NewRound or a Searching GameState and navigate back into the game
  useEffect(() => {
    if (!socket) return;
    const handler = (event: MessageEvent) => {
      try {
        const message = JSON.parse((event.data as string) || '{}');
        if (message.type === 'NewRound') {
          navigate('../game', { state: { target: message.target } });
        } else if (message.type === 'GameOver') {
          // If we reach true game over while on leaderboard, update rendering
          // by forcing a refresh to include final leaderboard
          // (In a full app we'd lift this to a store)
          navigate('.', { replace: true, state: { leaderboard: message.leaderboard, gameOver: true } });
        } else if (message.type === 'GameState') {
          // If we missed NewRound (e.g., due to reconnect timing), the authoritative state
          // will be Searching with a target. Navigate based on that.
          if (message.phase === 'Searching' && message.target) {
            navigate('../game', { state: { target: message.target } });
          }
        }
      } catch (_) {
        // ignore malformed
      }
    };
    socket.addEventListener('message', handler);
    return () => socket.removeEventListener('message', handler);
  }, [socket, navigate]);

  return (
    <div className="ui-container">
      <div className="ui-stack lg">
        <h2 className="ui-heading">{isGameOver ? 'Final Leaderboard' : 'Round Leaderboard'}</h2>
        {targetName && !isGameOver && (
          <div style={{ opacity: 0.75 }}>Next up: {targetName}</div>
        )}
        <div className="ui-card">
          <div className="ui-card-body">
            <div className="ui-stack sm">
              {rows.length === 0 && <div>No scores yet</div>}
              {rows.map((r) => (
                <div key={r.name} className="ui-flex" style={{ justifyContent: 'space-between' }}>
                  <span>{r.name}</span>
                  <span className="ui-tag">{r.score}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          <button className="ui-btn outline" onClick={() => navigate('..')}>Rejoin Lobby</button>
        </div>
      </div>
    </div>
  );
};

export default Leaderboard;

