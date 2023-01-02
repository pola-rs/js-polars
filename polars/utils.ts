export function waitForMsgType(target: any, type: any) {
  return new Promise(resolve => {
    target.addEventListener('message', function onMsg(event: any) {
      if (event.data == null || event.data.type !== type) return;
      target.removeEventListener('message', onMsg);
      resolve(event);
    });
  });
}
