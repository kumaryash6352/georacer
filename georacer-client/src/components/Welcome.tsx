import React from 'react';
import { Link } from 'react-router-dom';
import NameInput from './NameInput';

const Welcome: React.FC = () => {
  return (
    <div className="ui-container">
      <div className="ui-stack lg">
        <h1 className="ui-heading" style={{ textAlign: 'center' }}>Welcome to GeoRacer!</h1>
        <p className="ui-subtle" style={{ textAlign: 'center' }}>
          GeoRacer is a fast-paced game about knowing your home, street, or town. Be the first to find and snap a picture of the object to score!
        </p>
        <div className="ui-card">
          <div className="ui-card-body">
            <div className="ui-stack">
              <NameInput />
              <div className="ui-row" style={{ justifyContent: 'center' }}>
                <Link to="/settings" className="ui-btn primary">Create Lobby</Link>
                <Link to="/join" className="ui-btn outline">Join Lobby</Link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Welcome;

