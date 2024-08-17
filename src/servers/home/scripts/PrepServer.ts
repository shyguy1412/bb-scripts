import { allocateRam, getRamCost } from '@/lib/System';
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};
import { prepServer } from '@/lib/Hack';

export function autocomplete({ servers }: { servers: string[]; }) {
  return servers;
}

export async function main(ns: NS) {
  while (true) {
    const [target, hosts] = await allocateRam(ns, {
      ram: getRamCost(ns, [
        "getServer",
        "scan",
      ]),
    }, ns => {
      return [
        ns
          .getServer(ns.args[0] as string),
        getAllServersUnsafe(ns)
          .map(s => ns.getServer(s))
          .filter(s => s.hasAdminRights && s.maxRam)
          .map(s => s.hostname)
      ] as const;
    });
    if (!hosts.length) throw new Error("could not get hosts");

    if (target.hackDifficulty == target.minDifficulty && target.moneyAvailable == target.moneyMax) return;
    await prepServer(ns, target.hostname, hosts);

  }
}