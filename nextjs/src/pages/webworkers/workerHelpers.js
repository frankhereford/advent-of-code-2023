export function postMessageToWorker(message) {
  self.postMessage({ action: 'statusUpdate', message });
}
