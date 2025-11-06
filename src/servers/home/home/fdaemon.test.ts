import { connect_to_fdaemon } from "@/servers/home/bin/service/fdaemon";
import { system_cycle } from "@/servers/home/bin/kernel";
import { enable_hot_reload } from "@/servers/home/bin/service/hmr-daemon";

export async function main(ns: NS) {

  enable_hot_reload(ns);
  
  const fdaemon = connect_to_fdaemon(ns);
  const [send, receive] = fdaemon;

  while (true) {
    const cycled = await system_cycle(ns);
    if (!cycled) { console.log("KERNEL TIMEOUT"); continue; };

    const sent = send("subscribe", {
      filename: "/etc/plasma/plasmaconf.json",
      event: "change"
    });

    if (!sent) continue;

    const fsEvent = await receive().catch(e => console.log(e));

    if (!fsEvent) {
      await ns.sleep(0);
      continue;
    }

    console.log(fsEvent);
  }

}