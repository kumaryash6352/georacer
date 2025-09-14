import React, { createContext, useContext, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import config from '../config';

interface WebSocketContextType {
  socket: WebSocket | null;
  sendMessage: (message: any) => void;
}

const WebSocketContext = createContext<WebSocketContextType | null>(null);

export const useWebSocket = () => {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error('useWebSocket must be used within a WebSocketProvider');
  }
  return context;
};

export const WebSocketProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { id: lobbyId } = useParams<{ id: string }>();
  const [socket, setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    if (lobbyId) {
      const ws = new WebSocket(`ws://${config.apiUrl}/ws/${lobbyId}`);
      setSocket(ws);

      return () => {
        ws.close();
      };
    }
  }, [lobbyId]);

  const sendMessage = (message: any) => {
    if (socket) {
      console.log("sending");
      socket.send(JSON.stringify(message));
    }
  };

  return (
    <WebSocketContext.Provider value={{ socket, sendMessage }}>
      {children}
    </WebSocketContext.Provider>
  );
};
