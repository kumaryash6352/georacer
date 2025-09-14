
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
                    const response = await fetch(`${config.apiUrl}/gameobject/image`, {
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
        <div>
            <h1>Add New Game Object</h1>
            <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Enter object name" />
            <CameraView ref={cameraRef} />
            <button onClick={handleSave}>Save</button>
        </div>
    );
};

export default AddObject;
