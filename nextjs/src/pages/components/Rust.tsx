import React, { useEffect, useState } from 'react';

const Rust: React.FC = () => {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);
    const [worker, setWorker] = useState<Worker | null>(null);
    const [result, setResult] = useState<string | null>(null);

    useEffect(() => {
        // Initialize the web worker
        const rustWorker = new Worker(new URL('../webworkers/rustWorker.tsx', import.meta.url), { type: 'module' });
        setWorker(rustWorker);

        rustWorker.onmessage = function (e) {
            //console.log("e", e)
            if (e.data.action === 'statusUpdate') {
                // Handle status updates here
                console.log('Status update from Rust:', e.data.message);
            } else if (e.data.result) {
                // Handle result
                setResult(e.data.result);
            }
        };

        // Cleanup the worker when the component unmounts
        return () => {
            rustWorker.terminate();
        };
    }, []);

    useEffect(() => {
        const initWasm = async () => {
            try {
                await import('../../rust/television'); // Adjust the path as needed
                setIsWasmLoaded(true);
            } catch (error) {
                console.error('Error initializing WASM:', error);
            }
        };

        initWasm();
    }, []);

    const handleGreet = () => {
        if (worker && isWasmLoaded) {
            worker.postMessage({ action: 'runSolution', value: 50000 }); // Example value
        }
    };

    return (
        <div>
            {isWasmLoaded ? (
                <>
                    <button onClick={handleGreet}>Run Solution in Worker</button>
                    {result && <p>Result: {result}</p>}
                </>
            ) : (
                'Loading WASM...'
            )}
        </div>
    );
};

export default Rust;
