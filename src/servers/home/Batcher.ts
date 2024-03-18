import { batch, calculateBatch, calculateBatchingTarget, prepServer } from '@/lib/Hack';
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};
import { allocateRam, getRamCost, getWorkerServer } from '@/lib/System';

const HACK_THREADS = 5;
const CONCURRENT_BATCHES = 100;
const MAX_BATCHES = 200_000;
const INCLUDE_HOME = false;

export function autocomplete({ servers }: { servers: string[]; }) {
  return servers;
}

export async function main(ns: NS) {
  'use getHackingLevel';
  'use getServerMaxRam';
  'use getServerUsedRam';
  ns.disableLog('ALL');
  ns.tail();
  const flags = ns.flags([['h', false]]);

  //Get servers that run the workers
  const hacknet = await allocateRam(ns, {
    ram: getRamCost(ns, ['scan', 'getServer'])
  }, ns => getAllServersUnsafe(ns).map(s => ns.getServer(s)).filter(s => s.maxRam && s.hasAdminRights && (s.hostname != 'home' || INCLUDE_HOME || flags.h)).map(s => s.hostname));

  //If target was specified, prep it
  if (typeof ns.args[0] == 'string') {
    ns.print(`TARGET WAS SPECIFIED: ${ns.args[0]}`);
    ns.print(`PREPPING: ${ns.args[0]}`);
    await prepServer(ns, ns.args[0], hacknet);
    ns.print(`${ns.args[0]} PREPPED`);
  };

  const queue: Promise<void>[] = [];//current batches
  while (true) {
    //Get servers that run the workers
    const hacknet = await allocateRam(ns, {
      ram: getRamCost(ns, ['scan', 'getServer'])
    }, ns => getAllServersUnsafe(ns).map(s => ns.getServer(s)).filter(s => s.maxRam && s.hasAdminRights && (s.hostname != 'home' || INCLUDE_HOME || flags.h)).map(s => s.hostname));

    ns.print(`HACKNET: [${hacknet.join(', ')}]`);

    //hacking level at time of batch calculations
    const playerLevel = ns.getHackingLevel();
    //Get the batch info for the best or specified target
    const target =
      typeof ns.args[0] == 'string' ?
        await calculateBatch(ns, ns.args[0], HACK_THREADS, queue) :
        await calculateBatchingTarget(ns, HACK_THREADS, queue);

    ns.print(`CALCULATED BATCHES FOR: ${target.server.hostname}`);
    //Now that we have our target, make sure its prepped.
    //This is duplicate when target was specified, but it wont matter if its prepped already
    ns.print(`PREPPING: ${target.server.hostname}`);
    await prepServer(ns, target.server.hostname, hacknet);
    ns.print(`${target.server.hostname} PREPPED`);
  
    let failed = false; //exit if batches desync
    let batchCount = 0; //counter for concurrent batches

    ns.print(`READY TO DISPATCH`);
    await queue.shift() //properly wait to get back into batching
    while (!failed && playerLevel == ns.getHackingLevel()) {
      //get possible worker hosts
      const hostChoices = hacknet.map(hostname => ({
        hostname,
        freeRam: ns.getServerMaxRam(hostname) - ns.getServerUsedRam(hostname)
      }));

      //find 3 hosts that can run the HGW workers
      const hosts = target.ram.map(ram => {
        const worker = getWorkerServer(hostChoices, ram);
        if (!worker) {
          return '';
        }
        worker.freeRam -= ram;
        return worker.hostname;
      }) as [string, string, string];

      if (!hosts.some(h => h == '') && queue.length < MAX_BATCHES) { //if all workers have a host
        queue.push(batch(ns, target, hosts).catch(e => void (console.log(e), failed = true)));
      } else {
        //await batch to finish
        await queue.shift();
      }
      if (batchCount > CONCURRENT_BATCHES) {
        await ns.sleep(0); //prevents hangs and keeps up performance
        batchCount = 0;
      }
      else {
        await Promise.resolve();
        batchCount++;
      };
    }
    ns.print("LEVEL UP! RECALCULATING BATCHES");
    if (INCLUDE_HOME || flags.h) await Promise.all(queue); //await ram on home
  }

}


