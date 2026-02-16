import { create_service_interface, register_as_service, Request } from "@/lib/syscalls/service";
import { system_cycle } from "@/home/bin/kernel";
import __META_FILENAME from "meta:filename";

type HMRData = Map<number, string>;

const __HMR_CACHE = "/tmp/__hmr_cache.json";

const [
  connect_to_hmr_daemon,
  create_request_channel
] = create_service_interface<Request, never>(__META_FILENAME);
export { connect_to_hmr_daemon };

export function enable_hot_reload(ns: NS) {
  const [write] = connect_to_hmr_daemon(ns);
  write("", "");
}

export async function main(ns: NS) {
  const run_daemon_cycle = create_hmr_daemon(ns);

  while (true) {
    const cycled = await system_cycle(ns);
    if (!cycled) continue;

    run_daemon_cycle();
  }
}


export function create_hmr_daemon(ns: NS) {
  register_as_service(ns, __META_FILENAME);

  const data: HMRData = new Map();

  const cache = JSON.parse(ns.read(__HMR_CACHE) || "[]");

  for (const [key, value] of cache) {
    data.set(key, value);
  }

  ns.rm(__HMR_CACHE);

  return () => hmr_daemon_cyle(ns, data);
}

function hmr_daemon_cyle(ns: NS, data: HMRData) {
  process_request_queue(ns, data);

  for (const [pid, content] of data) {
    const script = ns.getRunningScript(pid);
    if (!script) {
      data.delete(pid);
      continue;
    }

    const new_content = ns.read(script.filename);

    if (content == new_content) continue;

    console.log(`HOT RELOAD (${script.pid}): ${script.server}://${script.filename}`);

    if (pid == ns.pid) {
      const self = ns.self();
      ns.ramOverride(self.ramUsage + 2);
      ns.write(__HMR_CACHE, JSON.stringify(data.entries().toArray()), "w");
      ns["spawn"](self.filename, { threads: self.threads, spawnDelay: 0 }, ...self.args);
      continue;
    }

    ns.kill(pid);
    ns.exec(script.filename, script.server, script.threads, ...script.args);

  }
}

function process_request_queue(ns: NS, data: HMRData) {
  const request_channel = create_request_channel(ns);
  for (const request of request_channel) {

    const sender_script = ns.getRunningScript(request.sender);
    if (!sender_script) continue;
    const content = ns.read(sender_script.filename);

    if (!request.sender) console.log({ request });

    // console.log("NEW HMR LISTENER: " + request.sender);

    data.set(request.sender, content);
  }
}