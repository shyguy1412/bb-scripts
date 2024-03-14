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
  'use fileExists';
  'use scp';

  const threads = options.threads || 1;
  const ram = options.ram;
  const host = typeof options.host == 'string' ?
    options.host :
    options.host ? getWorkerServer(options.host.map(hostname => ({
      hostname, freeRam: ns.getServerMaxRam(hostname) - ns.getServerUsedRam(hostname)
    })), ram * threads)?.hostname : ns.getHostname();

  if (!host) throw new Error('RAM could not be allocated, no suitable host');
  if (!ns.fileExists('ram-allocator.js', host)) ns.scp('ram-allocator.js', host, 'home');
  const pid = ns.exec('ram-allocator.js', host, { ramOverride: ram, threads, temporary: true });
  if (!pid) throw new Error('RAM could not be allocated, script failed to start');

  return new Promise((resolve) => {
    (globalThis as any)[`ns-${pid}`] = [callback, resolve];
  });
}

export function getMaxThreads(server: Server, ram: number) {
  return Math.floor((server.maxRam - server.ramUsed) / ram);
};