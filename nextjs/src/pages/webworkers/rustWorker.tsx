/// <reference lib="webworker" />

declare const self: DedicatedWorkerGlobalScope;

let television: any; // Adjust the type as necessary

async function loadWasmModule() {
  const wasmModule = await import('../../rust/television'); // Dynamic import
  television = wasmModule;
}

self.onmessage = async (e: MessageEvent) => {
  if (e.data.action === 'runSolution') {
    try {
      if (!television) {
        await loadWasmModule();
      }
      const result = await television.solution(e.data.value);
      self.postMessage({ result });
    } catch (error) {
      self.postMessage({ error: 'error' });
    }
  } else if (e.data.action === 'statusUpdate') {
    // Forward status updates to the main thread
    self.postMessage({ action: 'statusUpdate', message: e.data.message });
  } else {
    self.postMessage({ action: 'genericUpdate', message: e.data.message });
  }

};
export { }; // Treat this file as a module
