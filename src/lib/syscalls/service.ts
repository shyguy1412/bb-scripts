import { alive, getSafePortHandle } from "@/lib/System";
import { system_cycle } from "@/servers/home/bin/kernel";

type Service = {
  port: number;
  service: string;
};

export type ResponseChannel<T> = (response: T) => void;

export type Response<T> = {
  service: string,
  data: T;
};

export type Message<T> = {
  sender: number,
  data: T;
};

export function create_service_interface<T, R>(service_name: string) {
  const connect_to_service =
    (ns: NS) => connect_to_service_interface<T, R>(ns, service_name);

  const create_response_channel =
    (ns: NS, port: number) => response_channel<R>(ns, service_name, port);

  return [connect_to_service, create_response_channel] as const;
}

export function connect_to_service_interface<T, R>(ns: NS, service_name: string) {
  const service = get_service(ns, service_name);

  const writeToService = (arg: T) => write_to_service(ns, service, arg);
  const readFromService = () => read_from_service(ns, service);

  return [writeToService, readFromService] as const;
}

export function response_channel<T>(ns: NS, service: string, port: number) {
  return (data: T) => {
    const response: Response<T> = {
      service,
      data
    };

    return ns.tryWritePort(port, response);;
  };
}


export function write_to_service<T>(ns: NS, service: Service, arg: T) {
  const message: Message<T> = {
    sender: ns.pid,
    data: arg
  };

  return ns.tryWritePort(service.port, message);
}

export async function read_from_service<R>(ns: NS, service: Service) {
  const port = getSafePortHandle(ns, ns.pid)!;
  const servicePort = service.port;

  while (true) {
    await system_cycle(ns);

    const hasDisconnected = !alive(ns) || service.port != servicePort;

    if (hasDisconnected) throw `${service.service} has disconnected`;

    if (port.peek().service != service.service) continue;

    return port.read().data;
  }
}

export function get_service(ns: NS, service: string): Service {
  return Object.defineProperties({ port: 0, service }, {
    port: {
      get: () => get_service_port(ns, service)
    },
  });
}

export function get_service_port(ns: NS, service: string): number {
  return +ns.read(`/run/${service}.txt`);
}

export function register_as_service(ns: NS, service?: string) {
  const filename = service ?? ns.self().filename.replace(/.*\/([^\.]*).*/, "$1");
  const pidFile = `/run/${filename}.txt`;

  const currently = +ns.read(pidFile);

  if (currently && currently != ns.pid && ns.isRunning(currently))
    throw new Error("another instance is already running");

  ns.write(pidFile, ns.pid + "", 'w');

  ns.atExit(() => ns.rm(pidFile), "__syscall_register_service");
}
