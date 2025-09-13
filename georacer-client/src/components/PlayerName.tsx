import React from 'react';

const PlayerName: React.FC<{ name: string }> = ({ name }) => {
    return <a>{name}</a>
}

export default PlayerName;
