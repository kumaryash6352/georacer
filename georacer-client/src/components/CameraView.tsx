import React, { useRef, useEffect, useImperativeHandle, forwardRef } from 'react';

const CameraView = forwardRef((props, ref) => {
    const videoRef = useRef<HTMLVideoElement>(null);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        async function getCamera() {
            if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
                try {
                    const stream = await navigator.mediaDevices.getUserMedia({ video: true });
                    if (videoRef.current) {
                        videoRef.current.srcObject = stream;
                    }
                } catch (err) {
                    console.error("Error accessing camera: ", err);
                }
            }
        }
        getCamera();
    }, []);

    useImperativeHandle(ref, () => ({
        takePicture: () => {
            if (videoRef.current && canvasRef.current) {
                const context = canvasRef.current.getContext('2d');
                if (context) {
                    const width = videoRef.current.videoWidth;
                    const height = videoRef.current.videoHeight;
                    canvasRef.current.width = width;
                    canvasRef.current.height = height;
                    context.drawImage(videoRef.current, 0, 0, width, height);
                    return canvasRef.current.toDataURL('image/png');
                }
            }
            return null;
        }
    }));

    return (
        <div>
            <video ref={videoRef} autoPlay playsInline style={{ width: '100%' }} />
            <canvas ref={canvasRef} style={{ display: 'none' }} />
        </div>
    );
});

export default CameraView;
