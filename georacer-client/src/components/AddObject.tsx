
import React, { useState, useRef } from 'react';
import CameraView from './CameraView';
import config from '../config';

const AddObject: React.FC = () => {
    const [name, setName] = useState('');
    const cameraRef = useRef<{ takePicture: () => string | null }>(null);

    const handleSave = async () => {
        if (cameraRef.current) {
            const image = cameraRef.current.takePicture();
            if (image) {
                try {
                    const response = await fetch(`http://${config.apiUrl}/gameobject/image`, {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ name, image }),
                    });
                    if (response.ok) {
                        alert('Object saved successfully!');
                        setName('');
                    } else {
                        alert('Failed to save object.');
                    }
                } catch (error) {
                    console.error('Error saving object:', error);
                    alert('Failed to save object.');
                }
            }
        }
    };

    return (
        <div className="ui-container">
            <div className="ui-stack lg">
                <h2 className="ui-heading">Add New Game Object</h2>
                <div className="ui-card">
                    <div className="ui-card-body">
                        <div className="ui-stack">
                            <input className="ui-input" value={name} onChange={(e) => setName(e.target.value)} placeholder="Enter object name" />
                            <CameraView ref={cameraRef} />
                            <button onClick={handleSave} className="ui-btn primary">Save</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AddObject;
