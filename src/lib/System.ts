import { NetscriptPort } from "NetscriptDefinitions";

export function alive(ns: NS): boolean {
  try {
    return !!ns.self().pid;
  } catch {
    return false;
  }
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
