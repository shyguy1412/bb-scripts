/**
 * 
 * @param {ns} ns 
 */
export async function main(ns) {
  return new Promise((r) => {
    globalThis['ns-' + ns.pid] = [ns, r];
  });
}