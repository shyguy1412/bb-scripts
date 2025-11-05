import { enable_hot_reload } from "@/lib/syscalls/hot_reload";
import { alive } from "@/lib/System";
import { connect_to_fdaemon } from "@/servers/home/bin/service/fdaemon";
import { system_cycle } from "@/servers/home/bin/kernel";

export async function main(ns: NS) {

  enable_hot_reload(ns);
  const fdaemon = connect_to_fdaemon(ns);
  const [send, receive] = fdaemon;

  while (true) {
    await system_cycle(ns);


    const sent = send({
      action: "subscribe",
      filename: "/etc/plasma/plasmaconf.json",
      event: "change"
    });

    if (!sent) continue;

    const fsEvent = await receive().catch(e => console.log(e));

    if(!fsEvent) {
      await ns.sleep(0);
      continue;
    }

    console.log(fsEvent);
  }

}