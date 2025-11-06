import { create_service_interface, Request, register_as_service, ResponseChannel } from "@/lib/syscalls/service";
import { getSafePortHandle } from "@/lib/System";
import { system_cycle } from "@/servers/home/bin/kernel";
import __META_FILENAME from "meta:filename";

export type FSEvent = {
  type: "change",
  filename: string;
  content: string;
};

export type SubscribeRequest = Request<"subscribe", {
  filename: string,
  event: FSEvent["type"];
}>;

export type Subscriber = SubscribeRequest["data"] & {
  send: ResponseChannel<FSEvent>;
};

export type UnsubscribeRequest = Request<"unsubscribe", {
  filename: string,
  event: FSEvent["type"];
}>;

export type FSDaemonRequest = SubscribeRequest | UnsubscribeRequest;

export type FsDaemonData = {
  listeners: Map<string, Map<number, Subscriber>>;
  fileCache: Map<string, string>;
};

const [
  connect_to_fdaemon,
  create_request_channel,
  create_response_channel
] = create_service_interface<FSDaemonRequest, FSEvent>(__META_FILENAME);
export { connect_to_fdaemon };


export async function main(ns: NS) {
  const run_daemon_cycle = create_fdaemon(ns);

  while (true) {
    const cycled = await system_cycle(ns);
    if (!cycled) continue;

    run_daemon_cycle();
  }
}

export function create_fdaemon(ns: NS) {
  register_as_service(ns, __META_FILENAME);

  const data = {
    listeners: new Map<string, Map<number, Subscriber>>(),
    fileCache: new Map<string, string>(),
  };

  return () => fdaemon_cycle(ns, data);
}

export function fdaemon_cycle(ns: NS, data: FsDaemonData) {

  process_request_queue(ns, data);

  for (const [filename, content] of data.fileCache.entries()) {
    const has_listeners_left = collect_unused_file_listeners(ns, filename, data);
    if (!has_listeners_left) continue;

    const fileListeners = data.listeners.get(filename);
    if (!fileListeners) continue;

    const newContent = ns.read(filename);

    if (content == newContent) continue;
    data.fileCache.set(filename, newContent);

    const event: FSEvent = {
      type: "change",
      filename,
      content: newContent
    };

    for (const [, subscriber] of fileListeners) {
      subscriber.send(event);
    }
  }
};

//returns if there are any listeners left after collection
function collect_unused_file_listeners(ns: NS, filename: string, data: FsDaemonData): boolean {
  const fileListeners = data.listeners.get(filename);

  fileListeners?.keys()
    .filter(pid => !ns.isRunning(pid))
    .forEach(listener => fileListeners.delete(listener));

  if (!fileListeners || fileListeners.size == 0) {
    data.fileCache.delete(filename);
    return false;
  };

  return true;
}

function process_request_queue(ns: NS, data: FsDaemonData) {
  const request_channel = create_request_channel(ns);

  for (const request of request_channel) {
    switch (request.type) {
      case "subscribe":
        handle_subscribe_request(ns, data, request);
        break;
      case "unsubscribe":
        handle_unsubscribe_request(ns, data, request);
        break;
    }
  }

}

function handle_unsubscribe_request(ns: NS, data: FsDaemonData, msg: UnsubscribeRequest) {
  const fileListeners = data.listeners.get(msg.data.filename);
  if (!fileListeners) return;
  fileListeners.delete(msg.sender);
}

function handle_subscribe_request(ns: NS, data: FsDaemonData, msg: SubscribeRequest) {
  if (!data.listeners.has(msg.data.filename))
    data.listeners.set(msg.data.filename, new Map());

  const fileListeners = data.listeners.get(msg.data.filename)!;

  fileListeners.set(msg.sender, {
    ...msg.data,
    send: create_response_channel(ns, msg.sender)
  });

  data.fileCache.set(msg.data.filename, ns.read(msg.data.filename));
}