import { allocateRam, getRamCost } from '@/lib/System';
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};

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
          .getServer(ns.args[0] as string ?? 'foodnstuff'),
        getAllServersUnsafe(ns)
          .map(s => ns.getServer(s))
          .filter(s => s.hasAdminRights && s.maxRam)
      ] as const;
    });
    if (!hosts.length) throw new Error("could not get hosts");

    if (target.hackDifficulty != target.minDifficulty) {

      await Promise.allSettled(hosts.map(server => {
        const ram = getRamCost(ns, ["weaken"]);
        const threads = Math.floor((server.maxRam - server.ramUsed) / ram) || 1;
        return allocateRam(ns, { host: hosts.map(s => s.hostname), ram, threads }, ns => ns.weaken(target.hostname));
      }));
    } else {
      await Promise.allSettled(hosts.map(server => {
        const ram = getRamCost(ns, ["grow"]);
        const threads = Math.floor((server.maxRam - server.ramUsed) / ram) || 1;
        return allocateRam(ns, { host: hosts.map(s => s.hostname), ram, threads }, ns => ns.grow(target.hostname));
      }));
    }

  }
}