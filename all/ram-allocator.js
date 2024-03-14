export async function main(ns) {
  const [task, report] = globalThis['ns-' + ns.pid];
  delete globalThis['ns-' + ns.pid];
  const value = await task(ns);
  ns.atExit(() => report(value));
}