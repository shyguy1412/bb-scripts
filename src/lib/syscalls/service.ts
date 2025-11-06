import { alive, getSafePortHandle } from "@/lib/System";
import { system_cycle } from "@/servers/home/bin/kernel";

type Service = {
  port: number;
  service: string;
};

export type ResponseChannel<T> = (response: T) => void;

export type Response<T = any> = {
  service: string,
  data: T;
};

export type Request<T extends string = string, D = any> = {
  type: T,
  sender: number,
  data: D;
};

export function create_service_interface<S extends Request, R>(service_name: string) {
  const connect_to_typed_service =
    (ns: NS) => connect_to_service<S, Response<R>>(ns, service_name);

  const create_typed_request_channel =
    (ns: NS) => create_request_channel<S>(ns);

  const create_typed_response_channel =
    (ns: NS, port: number) => create_response_channel<R>(ns, service_name, port);

  return [
    connect_to_typed_service,
    create_typed_request_channel,
    create_typed_response_channel
  ] as const;
};

export function connect_to_service<S extends Request, R extends Response>(ns: NS, service_name: string) {

  const service = get_service(ns, service_name);

  const typed_write_to_service = (type: S["type"], data: S["data"]) => write_to_service(ns, service, type, data);
  const typed_read_from_service = () => read_from_service<R>(ns, service);

  return [
    typed_write_to_service,
    typed_read_from_service
  ] as const;
}

export function* create_request_channel<S extends Request>(ns: NS): Generator<S, void, undefined> {
  const port = getSafePortHandle(ns, ns.pid)!;
  while (!port.empty()) yield port.read();
}

export function create_response_channel<R>(ns: NS, service: string, port: number) {
  return (data: R) => {
    const response: Response<R> = {
      service,
      data
    };

    return ns.tryWritePort(port, response);;
  };
}

export function write_to_service<T extends string, D>(ns: NS, service: Service, type: T, arg: D) {
  const message: Request<T, D> = {
    type,
    sender: ns.pid,
    data: arg
  };

  return ns.tryWritePort(service.port, message);
}

export async function read_from_service<R>(ns: NS, service: Service) {
  const port = getSafePortHandle(ns, ns.pid)!;
  const servicePort = service.port;

  while (true) {
    const cycled = await system_cycle(ns);
    if (!cycled) continue;

    const hasDisconnected = !alive(ns) || service.port != servicePort;

    if (hasDisconnected) throw `${service.service} has disconnected`;

    if (port.peek().service != service.service) continue;

    return port.read().data as R;
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
