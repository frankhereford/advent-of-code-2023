import React, { useEffect, useState } from 'react';
import * as television from '../../rust/television';

const Rust: React.FC = () => {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    useEffect(() => {
        const initWasm = async () => {
            try {
                await television.init(); // Attempt to initialize the WASM module
            } catch (error) {
                console.error('Error initializing WASM:', error);
            }
            setIsWasmLoaded(typeof television.greet === 'function'); // Check if the greet function is available
        };

        initWasm();
    }, []);

    const handleGreet = () => {
        if (isWasmLoaded) {
            television.solution(); // Call the greet function from the WASM module
        }
    };

    return (
        <div>
            {isWasmLoaded ? (
                <button onClick={handleGreet}>Greet</button>
            ) : (
                'Loading WASM...'
            )}
        </div>
    );
};

export default Rust;
