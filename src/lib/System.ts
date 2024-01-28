import { Server } from "NetscriptDefinitions";

export const RAM_ALLOCATOR_SCRIPT = /*javascript */`
/**
 * @param {NS} ns
 */
export async function main(ns) {
  globalThis['ns-' + ns.pid] = ns;
  ns.writePort(ns.pid, "");
  ns.clearPort(ns.pid);
  await ns.getPortHandle(ns.pid).nextWrite();
}
`;

export async function sleep(ms: number) {
  await new Promise<void>(resolve => setTimeout(() => resolve(), ms));
}

export function getRamCost(ns: NS, functions: string[], threads = 1) {
  return (1.6 + functions.reduce((a, b) => a + ns.getFunctionRamCost(b), 0)) * threads;
}

type AllocateOptions = {
  ram: number;
  threads?: number;
  host?: string;
};
export async function allocateRam<T = any>(ns: NS, options: AllocateOptions, callback: (ns: NS) => T): Promise<T> {
  'use exec';
  'use getHostname';
  'use getServerMaxRam';
  'use getServerUsedRam';

  const ram = options.ram;
  const host = options.host ?? ns.getHostname();
  const threads = options.threads ?? 1;
  if (ns.getServerMaxRam(host) - ns.getServerUsedRam(host) < ram * threads)
    console.log("WAITING FOR RAM");

  while (ns.getServerMaxRam(host) - ns.getServerUsedRam(host) < ram * threads) {
    await sleep(100);
  }

  const pid = ns.exec('ram-allocator.js', host, { ramOverride: ram, threads });
  if (!pid) throw new Error('RAM could not be allocated');

  await sleep(0); //await script execution
  await sleep(0); //await script execution again to account for potential compile

  const alloc = globalThis[`ns-${pid}`] as NS;
  ns.atExit(() => {
    ns.writePort(pid, ''); //free ram
    ns.clearPort(pid);
  });

  try {
    const result = await callback(alloc);

    ns.writePort(pid, ''); //free ram
    ns.clearPort(pid);

    await sleep(0); //wait for cleanup

    return result;

  } catch (e) {
    ns.writePort(pid, ''); //free ram
    ns.clearPort(pid);
    console.log(e);

    return undefined;
  }
}

export function getMaxThreads(server: Server, ram: number) {
  return Math.floor((server.maxRam - server.ramUsed) / ram);
};