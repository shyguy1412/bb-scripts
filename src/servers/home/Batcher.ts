import { batch, calculateBatch, calculateBatchingTarget, prepServer } from '@/lib/Hack';
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};
import { allocateRam, getRamCost, getWorkerServer } from '@/lib/System';
import { hostname } from 'os';

const HACK_THREADS = 1;

export function autocomplete({servers}:{servers:string[]}){
  return servers
}

export async function main(ns: NS) {
  'use getServerMaxRam';
  'use getServerUsedRam';

  const hacknet = await allocateRam(ns, {
    ram: getRamCost(ns, ['scan', 'getServer'])
  }, ns => getAllServersUnsafe(ns).map(s => ns.getServer(s)).filter(s => s.maxRam && s.hasAdminRights).map(s => s.hostname));

  if (typeof ns.args[0] == 'string') await prepServer(ns, ns.args[0], hacknet);

  const target =
    typeof ns.args[0] == 'string' ?
      await calculateBatch(ns, ns.args[0], HACK_THREADS) :
      await calculateBatchingTarget(ns, HACK_THREADS);

  let failed = false;
  const queue: Promise<any>[] = [];

  await prepServer(ns, target.server.hostname, hacknet);

  console.log(target);

  while (!failed) {
    const hostChoices = hacknet.map(hostname => ({
      hostname,
      freeRam: ns.getServerMaxRam(hostname) - ns.getServerUsedRam(hostname)
    }));

    const hosts = target.ram.map(ram => {
      const worker = getWorkerServer(hostChoices, ram);
      if (!worker) {
        return '';
      }
      worker.freeRam -= ram;
      return worker.hostname;
    }) as [string, string, string];

    if (!hosts.some(h => h == '')) {
      queue.push(batch(ns, target, hosts).catch(e => void (console.log(e), failed = true)));
    } else {
      await queue.shift();
    }
    await ns.sleep(0);
  }
}


