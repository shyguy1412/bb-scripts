import { connect_to_fdaemon } from "@/home/bin/service/fdaemon";
import { system_cycle } from "@/home/bin/kernel";
import { enable_hot_reload } from "@/home/bin/service/hmr-daemon";
import { get_service, register_as_service } from "@/lib/syscalls/service";
import __META_FILENAME from "meta:filename";

export async function main(ns: NS) {
  if (ns.args.includes("--replace")) {
    const service = get_service(ns, __META_FILENAME);
    if (service.port != 0) ns.kill(service.port);
  }

  register_as_service(ns);

  enable_hot_reload(ns);
  const fdaemon = connect_to_fdaemon(ns);
  const [send, receive] = fdaemon;

  while (true) {

    const cycled = await system_cycle(ns);
    if (!cycled) { console.log("KERNEL TIMEOUT"); continue; };

    const sent = send("subscribe", {
      path: "/etc/plasma",
      event: "change"
    });

    if (!sent) continue;

    const fsEvent = await receive().catch(e => void e); //ignore errors

    if (!fsEvent) continue;

    console.log(fsEvent);

    continue;
  }

}