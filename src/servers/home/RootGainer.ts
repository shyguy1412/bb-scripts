import { gainRoot } from "@/lib/Hack"  with {type: 'unsafe'};
import { getAllServers } from "@/lib/Network"  with {type: 'unsafe'};
import { allocateRam, getRamCost, sleep } from "@/lib/System";

export async function main(ns: NS) {
  ns.disableLog('ALL');

  const { servers } = await allocateRam(ns, {
    ram: getRamCost(ns, ['getServer', 'scan'])
  }, (ns) => {
    const servers = getAllServers(ns).map(s => ns.getServer(s)).filter(s => !s.hasAdminRights);
    return { servers };
  });

  ns.print('SERVERS TO CRACK:');
  ns.print(...servers.map(s => `${s.hostname}\n`));

  while (servers.length) {
    for (const server of servers) {

      const gainedRoot = await allocateRam(ns, {
        ram: getRamCost(ns, [])
      }, ns => gainRoot(ns, server.hostname)).catch(_ => false);

      if (gainedRoot) {
        queueMicrotask(() => {
          servers.splice(servers.indexOf(server), 1);
        });
        ns.print("GAINED ROOT: " + server.hostname);
      };
    }
    await sleep(1000);
  }
  await sleep(0);
}