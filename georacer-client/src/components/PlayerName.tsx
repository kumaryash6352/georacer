import React from 'react';
import { useName } from '../contexts/NameContext';

const PlayerName: React.FC<{ name: string }> = ({ name }) => {
  let myName = useName();
  const isMe = name === myName.name;
  return (
    <span className={`ui-tag ${isMe ? 'me' : ''}`}>{name}{isMe ? ' (you!)' : ''}</span>
  );
}

export default PlayerName;
