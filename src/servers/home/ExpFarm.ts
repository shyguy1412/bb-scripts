import { fullyWeakenServer } from '@/lib/Hack';
import { allocateRam, getRamCost } from '@/lib/System';

export async function main(ns: NS) {
  const target = ns.args[0] as string ?? 'foodnstuff';

  await fullyWeakenServer(ns, target);

  while (true) { 
    const server = await allocateRam(ns, {
      ram: getRamCost(ns, [
        "getServer",
        "getHostname",
      ]),
    }, ns => { 
      return ns.getServer(ns.getHostname());
    });
    if (!server) throw new Error("could not get server"); 
    const ram = getRamCost(ns, ["grow"]);
    const threads = Math.floor((server.maxRam - server.ramUsed) / ram) || 1;
    await allocateRam(ns, { ram, threads }, ns => ns.grow(target));
  }
}