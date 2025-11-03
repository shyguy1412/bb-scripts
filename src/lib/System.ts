import { NetscriptPort, Server } from "NetscriptDefinitions";
//@ts-expect-error: invalid type due to with text assertion
import RamAllocatorScript from "@/lib/ram-allocator" with {type: 'text'};

export async function sleep(ms: number) {
  await new Promise<void>(resolve => setTimeout(() => resolve(), ms));
}

export function getRamCost(ns: NS, functions: string[], threads = 1) {
  return (1.6 + functions.reduce((a, b) => a + ns.getFunctionRamCost(b), 0)) * threads;
}

export function getSafePortHandle(ns: NS, port: number): NetscriptPort | undefined {
  if (port < 1) return;
  return {
    write: (value: any) => ns.writePort(port, value),
    tryWrite: (value: any) => ns.tryWritePort(port, value),
    nextWrite: () => ns.nextPortWrite(port),
    read: () => ns.readPort(port),
    peek: () => ns.peek(port),
    full: () => ns.getPortHandle(port).full(),
    empty: () => ns.getPortHandle(port).empty(),
    clear: () => ns.clearPort(port)
  };
}

type AllocateOptions = {
  ram: number;
  threads?: number;
  host?: string | string[];
};

export function getWorkerServer(hosts: { hostname: string, freeRam: number; }[], ram: number): typeof hosts[number] | undefined {
  return hosts.reduce<typeof hosts[number] | undefined>((prev, cur) => {
    if (prev) return prev.freeRam > ram ? prev : cur;

    if (cur.freeRam > ram) return cur;

    return undefined;
  }, undefined);
}

export function allocateRam<T = any>(ns: NS, options: AllocateOptions, callback: (ns: NS) => T): Promise<T> {
  'use exec';
  'use getHostname';
  'use getServerMaxRam';
  'use getServerUsedRam';

  const threads = options.threads || 1;
  const ram = options.ram;
  const host = typeof options.host == 'string' ?
    options.host :
    options.host ? getWorkerServer(options.host.map(hostname => ({
      hostname, freeRam: ns.getServerMaxRam(hostname) - ns.getServerUsedRam(hostname)
    })), ram * threads)?.hostname : ns.getHostname();

  return new Promise((resolve, reject) => {
    if (!host) return reject('RAM could not be allocated, no suitable host');
    ns.write('ram-allocator.js', RamAllocatorScript, 'w');

    const pid = ns.exec('ram-allocator.js', host, { ramOverride: ram, threads, temporary: true });

    if (!pid) return reject('RAM could not be allocated, script failed to start');

    (globalThis as any)[`ns-${pid}`] = [callback, resolve];
  });
}

export function getMaxThreads(server: Pick<Server, 'maxRam' | 'ramUsed'>, ram: number) {
  return Math.floor((server.maxRam - server.ramUsed) / ram);
};