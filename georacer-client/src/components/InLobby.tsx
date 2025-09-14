import { Outlet } from 'react-router-dom';
import { WebSocketProvider } from '../contexts/WebSocketContext';

const InLobby = () => {
  return (
    <WebSocketProvider>
      <Outlet />
    </WebSocketProvider>
  );
};

export default InLobby;
