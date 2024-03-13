export async function main(ns) {
  const [task, report] = globalThis['ns-' + ns.pid];
  delete globalThis['ns-' + ns.pid];
  report(await task(ns));
}