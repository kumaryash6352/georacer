import React from 'react';
import { Link } from 'react-router-dom';

const Welcome: React.FC = () => {
  return (
    <div>
      <h1>Welcome to Georacer!</h1>
      <p>GeoRacer is a fast paced game about knowing your home, street, or town. Be the first to find and snap a picture of the object or anything like it to score!</p>
      <Link to="/settings"><button>Create Lobby</button></Link>
      <Link to="/join"><button>Join Lobby</button></Link>
    </div>
  );
};

export default Welcome;

