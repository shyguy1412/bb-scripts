import { fullyGrowServer, fullyWeakenServer } from "@/lib/Hack";
import { getAllServers } from "@/lib/Network" with {type: 'unsafe'};
import { allocateRam, getRamCost, sleep } from "@/lib/System";

export async function main(ns: NS) {
  'use getServerMaxRam';
  'use getServerUsedRam';

  const { servers, host } = await allocateRam(ns, {
    ram: getRamCost(ns, [
      "scan",
      "getServer",
      "getHackingLevel",
      "getHostname",
    ])
  }, (ns) => {
    const host = ns.getHostname();
    const servers = getAllServers(ns)
      .map(s => ns.getServer(s))
      .filter(s => s.hasAdminRights && s.hackDifficulty < ns.getHackingLevel() && !s.purchasedByPlayer);
    return { servers, host };
  });
  
  try {
    await Promise.all(servers.map(async s => {
      await fullyWeakenServer(ns, s.hostname);
      await fullyGrowServer(ns, s.hostname);
      await fullyWeakenServer(ns, s.hostname);
    }));
  } catch (e) { console.log(e); }

}