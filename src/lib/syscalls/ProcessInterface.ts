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
        if (port.empty()) await new Promise<void>(async (resolve, reject) => {
          let hasResolved = false;

          const subResolve = () => hasResolved = true;

          port.nextWrite().then(subResolve);

          while (!hasResolved) {
            await ns.sleep(0);
            if (connection.port == 0) return reject(`${process} has disconnected`);
          }

          return resolve();
        });

        if (port.peek().process != process) {
          await ns.sleep(0);
          continue;
        }

        const msg = port.read();
        return msg.payload;
      }
    };

    return [
      write,
      read
    ] as const;
  };
}

export function awaitDisconnect() {

}

export function connectToProcess(ns: NS, process: string): Connection {
  return Object.defineProperties({ port: 0 }, {
    port: {
      get: () => +ns.read(`/run/${process}.txt`)
    }
  });
}