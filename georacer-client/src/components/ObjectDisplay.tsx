import React from 'react';

interface ObjectDisplayProps {
  target: {
    image_b64: string;
  };
}

const ObjectDisplay: React.FC<ObjectDisplayProps> = ({ target }) => {
  if (!target) {
    return <div style={{ height: '25vh', borderRadius: 16, background: 'rgba(255,255,255,0.08)' }} />;
  }

  return (
    <div style={{ borderRadius: 16, overflow: 'hidden', boxShadow: 'var(--ui-shadow-md)' }}>
      <img
        src={`${target.image_b64}`}
        alt="Object to find"
        style={{ width: '100%', height: '25vh', objectFit: 'cover', display: 'block' }}
      />
    </div>
  );
};

export default ObjectDisplay;

