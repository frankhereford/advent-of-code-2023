export function postMessageToWorker(do_print, message) {
  //console.log("do_print", do_print, "message", message)
  if (do_print) {
    console.log(message)
    self.postMessage({ action: 'statusUpdate', message });
  }
}