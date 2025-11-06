import { get_service, get_service_port, read_from_service, register_as_service } from "@/lib/syscalls/service";
import META_FILENAME from "meta:filename";
import { create_fdaemon } from "@/servers/home/bin/service/fdaemon";
import { create_hmr_daemon, enable_hot_reload } from "@/servers/home/bin/service/hmr-daemon";
import __META_FILENAME from "meta:filename";
import { getSafePortHandle } from "@/lib/System";

export const CYCLE_TIMEOUT = 5000;

export async function system_cycle(ns: NS) {
  const service_port = get_service_port(ns, META_FILENAME);
  if (!service_port) throw new Error(`${META_FILENAME} is not running`);

  const timeout = new Promise((_, reject) => setTimeout(reject, CYCLE_TIMEOUT));
  const cycle = new Promise(async resolve => {
    while (true) {
      await ns.nextPortWrite(service_port);
      const msg = ns.peek(service_port);
      if (msg.service != __META_FILENAME) continue;
      resolve(true);
    }
  });

  return Promise.race([cycle, timeout])
    .then(_ => true)
    .catch(_ => false);
}

export async function main(ns: NS) {
  if (ns.args.includes("--replace")) {
    const service = get_service(ns, __META_FILENAME);
    if (service.port != 0) ns.kill(service.port);
  }

  register_as_service(ns);

  const fdaemon = create_fdaemon(ns);
  const hmr_daemon = create_hmr_daemon(ns);

  enable_hot_reload(ns);

  const port = getSafePortHandle(ns, ns.pid)!;

  let last_uuid = "";

  while (true) {

    fdaemon();
    hmr_daemon();

    //if queue is empty for this cycle is empty
    if (port.empty() || port.peek().service == __META_FILENAME) {
      port.write({ service: __META_FILENAME, uuid: crypto.randomUUID() });
      await ns.asleep(0);
      continue;
    }

    const unprocessed_request = port.peek();

    if (unprocessed_request.uuid == last_uuid) {
      console.log("POP", port.read()); // pop unprocessed request
    }

    last_uuid = unprocessed_request.uuid;

    continue;
  }
}