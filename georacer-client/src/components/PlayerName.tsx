import React from 'react';
import { useName } from '../contexts/NameContext';

const PlayerName: React.FC<{ name: string }> = ({ name }) => {
    let myName = useName();
    return <a>{name}{name == myName.name ? " (you!)" : ""}</a>
}

export default PlayerName;
