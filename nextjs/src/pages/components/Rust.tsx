import React, { useEffect, useState } from 'react';

interface RustProps {
    onUpdate: (update: string) => void;
    day: number;
}

const Rust: React.FC<RustProps> = ({ onUpdate, day }) => {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);
    const [worker, setWorker] = useState<Worker | null>(null);

    useEffect(() => {
        // Initialize the web worker
        const rustWorker = new Worker(new URL('../webworkers/rustWorker.tsx', import.meta.url), { type: 'module' });
        setWorker(rustWorker);

        rustWorker.onmessage = function (e) {
            if (e.data.action === 'statusUpdate') {
                onUpdate(e.data.message + "\n");
            } else if (e.data.result) {
                // Handle result
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
                await import('../../rust/television');
                setIsWasmLoaded(true);
            } catch (error) {
                console.error('Error initializing WASM:', error);
            }
        };

        initWasm();
    }, []);

    useEffect(() => {
        if (worker && isWasmLoaded) {
            handleGreet();
        }
    }, [worker, isWasmLoaded]); // Dependency array includes worker and isWasmLoaded

    const handleGreet = () => {
        if (worker) {
            worker.postMessage({ action: 'runSolution', value: day});
        }
    };

    // Return an empty fragment
    return <></>;
};

export default Rust;
