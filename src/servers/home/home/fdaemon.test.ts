import { hotReload } from "@/lib/syscalls/HotReload";
import { connect } from "@/servers/home/bin/fdaemon";

export async function main(ns: NS) {

  hotReload(ns);
  const fdaemon = connect(ns);
  const [send, receive] = fdaemon;

  while (true) {
    await ns.sleep(0);

    const sent = send({
      action: "subscribe",
      filename: "/etc/plasma/plasmaconf.json",
      listener: ns.pid,
      event: "change"
    });

    if (!sent) continue;

    const fsEvent = await receive().catch(e => console.log(e));

    console.log(fsEvent);
  }

}