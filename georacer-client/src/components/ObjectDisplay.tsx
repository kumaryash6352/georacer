import React from 'react';

interface ObjectDisplayProps {
  target: {
    image_b64: string;
  };
}

const ObjectDisplay: React.FC<ObjectDisplayProps> = ({ target }) => {
  if (!target) {
    return <div>Loading...</div>;
  }

  return (
    <img
      src={`${target.image_b64}`}
      alt="Object to find"
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '25vh',
        objectFit: 'contain',
      }}
    />
  );
};

export default ObjectDisplay;

