import { Server } from "NetscriptDefinitions";

export async function sleep(ms: number) {
  await new Promise<void>(resolve => setTimeout(() => resolve(), ms));
}

export function getRamCost(ns: NS, functions: string[], threads = 1) {
  return (1.6 + functions.reduce((a, b) => a + ns.getFunctionRamCost(b), 0)) * threads;
}

type AllocateOptions = {
  ram: number;
  threads?: number;
  host?: string | string[];
};

export async function allocateRam<T = any>(ns: NS, options: AllocateOptions, callback: (ns: NS) => T): Promise<T> | undefined {
  'use exec';
  'use getHostname';
  'use getServerMaxRam';
  'use getServerUsedRam';

  const ram = options.ram;
  const host = typeof options.host == 'string' ?
    options.host :
    options.host?.reduce<string | undefined>((prev, cur) => {
      const freeRam = ns.getServerMaxRam(cur) - ns.getServerUsedRam(cur);
      if (!prev) {
        if (freeRam >= ram) return cur;
        return undefined;
      }
      const prevFreeRam = ns.getServerMaxRam(prev) - ns.getServerUsedRam(prev);
      return prevFreeRam >= ram ? prev : cur;
    }, undefined)
    ?? ns.getHostname();
  const threads = options.threads ?? 1;
  if (ns.getServerMaxRam(host) - ns.getServerUsedRam(host) < ram * threads)
    console.log("WAITING FOR RAM");

  while (ns.getServerMaxRam(host) - ns.getServerUsedRam(host) < ram * threads) {
    await sleep(100);
  }

  const pid = ns.exec('ram-allocator.js', host, { ramOverride: ram, threads, temporary: true });
  if (!pid) throw new Error('RAM could not be allocated');

  while (!globalThis[`ns-${pid}`]) await sleep(0);

  const [alloc, exit] = globalThis[`ns-${pid}`] as [NS, () => void];
  delete globalThis[`ns-${pid}`];

  return await Promise.resolve(callback(alloc))
    .catch(e => console.log(e) as undefined)
    .finally(exit);
}

export function getMaxThreads(server: Server, ram: number) {
  return Math.floor((server.maxRam - server.ramUsed) / ram);
};