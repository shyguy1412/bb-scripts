import { create_service_interface } from "@/lib/syscalls/service";
import { system_cycle } from "@/servers/home/bin/kernel";
import __META_FILENAME from "meta:filename";

const [
  connect_to_hmr_daemon,
  create_request_channel,
  create_response_channel
] = create_service_interface<string, never>(__META_FILENAME);
export { connect_to_hmr_daemon };

export async function main(ns: NS) {
  const run_daemon_cycle = create_hmr_daemon(ns);

  while (true) {
    const cycled = await system_cycle(ns);
    if (!cycled) continue;

    run_daemon_cycle();
  }
}

function create_hmr_daemon(ns: NS) {
  return () => hmr_daemon_cyle(ns);
}

function hmr_daemon_cyle(ns: NS) {

}