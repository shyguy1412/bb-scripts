import { allocateRam, getMaxThreads, getRamCost } from "@/lib/System";
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};

export function autocomplete({ servers }: { servers: string[]; }) {
  return servers;
}
export async function main(ns: NS) {
  'use getServerMaxRam';
  'use getServerUsedRam';

  while (true) {
    const hacknet = await allocateRam(ns, {
      ram: getRamCost(ns, ['scan', 'getServer'])
    }, ns => getAllServersUnsafe(ns).map(s => ns.getServer(s)).filter(s => s.maxRam && s.hasAdminRights && s.hostname != 'home').map(s => s.hostname));

    await Promise.allSettled(hacknet.map(host => {
      const threads = getMaxThreads({
        ramUsed: ns.getServerUsedRam(host),
        maxRam: ns.getServerMaxRam(host)
      }, getRamCost(ns, ['share']));
      if (!threads) return;
      return allocateRam(ns, { ram: getRamCost(ns, ['share']), host, threads }, ns => ns.share());
    }));

    await ns.sleep(0);
  }

}