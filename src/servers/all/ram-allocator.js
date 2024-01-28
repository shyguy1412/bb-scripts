/**
 * @param {NS} ns
 */
export async function main(ns) {
  globalThis[`ns-${ns.pid}`] = ns;
  ns.writePort(ns.pid, "");
  ns.clearPort(ns.pid);
  await ns.getPortHandle(ns.pid).nextWrite();
  ns.exit();
}