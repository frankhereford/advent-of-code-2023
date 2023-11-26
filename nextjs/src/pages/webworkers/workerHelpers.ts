export function postMessageToWorker(message: string): void {
  (self as DedicatedWorkerGlobalScope).postMessage({ action: 'statusUpdate', message });
}
