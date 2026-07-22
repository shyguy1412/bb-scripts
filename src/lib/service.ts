// import { system_cycle } from '@/home/bin/kernel';

import { Result } from '@/lib/result';

function alive(ns: NS): boolean {
    try {
        return !!ns.self().pid;
    } catch {
        return false;
    }
}

type Service = {
    port: number;
    name: string;
};

export type ResponseChannel<T> = (response: T) => void;

export type Response<T = any> = {
    uuid: string;
    service: string;
    data: T;
};

export type Request<T = string, D = any> = {
    uuid: string;
    service: string;
    type: T;
    sender: number;
    data: D;
};

export type RequestHandler<T> = (request: T) => void;

export function create_service_interface<S extends Request, R>(service_name: string) {
    const connect_to_typed_service = (ns: NS, from = ns.pid) =>
        connect_to_service<S, Response<R>>(ns, service_name, from);

    const create_typed_request_channel = (ns: NS, service_port = ns.pid) =>
        create_request_channel<S>(ns, service_name, service_port);

    const create_typed_response_channel = (ns: NS, port: number) =>
        create_response_channel<R>(ns, service_name, port);

    const register = (ns: NS, service_pid = ns.pid) => {
        const filename = service_name;
        const pidFile = `/run/${filename}.txt`;

        ns.write(pidFile, service_pid + '', 'w');
    };

    return [
        connect_to_typed_service,
        create_typed_request_channel,
        create_typed_response_channel,
        register,
    ] as const;
}

export function connect_to_service<S extends Request, R extends Response>(
    ns: NS,
    service_name: string,
    from: number,
) {
    const service = get_service(ns, service_name);

    type CollapsedArgs = S['data'] extends undefined ? [type: S['type']] :
        [type: S['type'], data: S['data']];

    const typed_write_to_service = (...[type, data]: CollapsedArgs) =>
        write_to_service(ns, service, type, data, from);
    const typed_read_from_service = () => read_from_service<R>(ns, service, from);
    const typed_read_from_service_non_blocking = () =>
        read_from_service_non_blocking<R>(ns, service, from);

    return [
        typed_write_to_service,
        typed_read_from_service,
        typed_read_from_service_non_blocking,
    ] as const;
}

export function* create_request_channel<S extends Request>(
    ns: NS,
    service_name: string,
    service_port: number,
): Generator<S, void, undefined> {
    while (!ns.getPortHandle(service_port).empty()) {
        if (ns.peek(service_port).service != service_name) {
            return;
        }
        yield ns.readPort(service_port);
    }
}

export function create_response_channel<R>(ns: NS, service: string, port: number) {
    return (data: R) => {
        const response: Response<R> = {
            uuid: crypto.randomUUID(),
            service,
            data,
        };

        return ns.tryWritePort(port, response);
    };
}

export function write_to_service<T extends string, D>(
    ns: NS,
    service: Service,
    type: T,
    arg: D,
    sender = ns.pid,
) {
    const message: Request<T, D> = {
        uuid: crypto.randomUUID(),
        type,
        service: service.name,
        sender,
        data: arg,
    };

    if (!service.port || +ns.read(`${service.name}.txt`) == service.port) {
        return false;
    }

    return ns.tryWritePort(service.port, message);
}
export function register_as_service(ns: NS, service?: string, service_pid = ns.pid) {
    const filename = service ?? ns.self().filename.replace(/.*?\/?([^\/]*)\..*/, '$1');
    const pidFile = `/run/${filename}.txt`;

    ns.write(pidFile, service_pid + '', 'w');
}

export async function read_from_service<R>(
    ns: NS,
    service: Service,
    port = ns.pid,
): Promise<Result<R, string>> {
    const servicePort = service.port;

    while (ns.peek(port).service != service.name) {
        if (!alive(ns)) {
            return Result.Err('script died');
        }

        if (service.port != servicePort) {
            return Result.Err(`${service.name} has disconnected`);
        }

        await ns.asleep(0);
    }

    return Result.Ok(ns.readPort(port));
}

export function read_from_service_non_blocking<R>(
    ns: NS,
    service: Service,
    port = ns.pid,
): Result<R, string> {
    const servicePort = service.port;

    if (ns.peek(port).service != service.name) {
        if (!alive(ns)) {
            return Result.Err('script died');
        }

        if (service.port != servicePort) {
            return Result.Err(`${service.name} has disconnected`);
        }

        return Result.Err(`data is not ready`);
    }

    return Result.Ok(ns.readPort(port));
}

export function get_service(ns: NS, service_name: string): Service {
    return Object.defineProperties({ port: 0, name: service_name }, {
        port: {
            get: () => get_service_port(ns, service_name),
        },
    });
}

export function get_service_port(ns: NS, service: string): number {
    return +ns.read(`/run/${service}.txt`);
}
