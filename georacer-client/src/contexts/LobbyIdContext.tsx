import { createContext, useContext, useState, ReactNode } from 'react';

interface LobbyIdContextType {
  lobbyId: string;
  setLobbyId: (id: string) => void;
}

const LobbyIdContext = createContext<LobbyIdContextType | undefined>(undefined);

export const LobbyIdProvider = ({ children }: { children: ReactNode }) => {
  const [lobbyId, setLobbyId] = useState<string>('');

  return (
    <LobbyIdContext.Provider value={{ lobbyId, setLobbyId }}>
      {children}
    </LobbyIdContext.Provider>
  );
};

export const useLobbyId = () => {
  const context = useContext(LobbyIdContext);
  if (!context) {
    throw new Error('useLobbyId must be used within a LobbyIdProvider');
  }
  return context;
};