import { getSafePortHandle } from "@/lib/System";

type Connection = { port: number; };

export function createProcessInterface<T, R>(process: string) {
  return (ns: NS) => {
    const connection = connectToProcess(ns, process);

    const write = (arg: T) => {
      const port = getSafePortHandle(ns, connection.port);
      if (!port || port.full()) return false;
      port.write(arg);
      return true;
    };

    const read = async (): Promise<R> => {
      const port = getSafePortHandle(ns, ns.pid)!;
      while (true) {
        await ns.sleep(0);
        if (connection.port == 0) throw `${process} has disconnected`;
        if (port.peek().process != process) continue;
        return port.read().payload;
      }
    };

    return [write, read] as const;
  };
}

export function connectToProcess(ns: NS, process: string): Connection {
  return Object.defineProperties({ port: 0 }, {
    port: {
      get: () => +ns.read(`/run/${process}.txt`)
    }
  });
}