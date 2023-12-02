export function postMessageToWorker(do_print, message) {
  //console.log("do_print", do_print, "message", message)
  if (do_print) {
    self.postMessage({ action: 'statusUpdate', message });
  }
}