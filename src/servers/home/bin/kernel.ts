import { get_service, register_as_service } from "@/lib/syscalls/service";
import META_FILENAME from "meta:filename";
import { create_fdaemon } from "@/servers/home/bin/service/fdaemon";
import { enable_hot_reload } from "@/lib/syscalls/hot_reload";

export const CYCLE_TIMEOUT = 5000;

export async function system_cycle(ns: NS) {
  const { port } = get_service(ns, META_FILENAME);
  if (!port) throw new Error(`${META_FILENAME} is not running`);

  const timeout = new Promise((_, reject) => setTimeout(reject, CYCLE_TIMEOUT));

  return Promise.race([ns.nextPortWrite(port), timeout]).then(_ => true).catch(_ => false);
}

export async function main(ns: NS) {
  register_as_service(ns);

  enable_hot_reload(ns);

  // ns.atExit(() => {
  //   //send a cycle after the scripts death
  //   //this prevents other scripts from immediatly awating the next cycle and getting stuck
  //   const port = ns.getPortHandle(ns.pid);
  //   setTimeout(() => {
  //     port.write({ process: META_FILENAME });
  //     port.clear();
  //   });
  // }, "__kernel_send_final_cycle");

  const fdaemon = create_fdaemon(ns);

  while (true) {
    await ns.sleep(0);

    fdaemon();

    ns.clearPort(ns.pid);
    ns.writePort(ns.pid, { process: META_FILENAME });

    // continue;
  }
}