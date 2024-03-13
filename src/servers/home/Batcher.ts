import { fullyGrowServer, fullyWeakenServer } from '@/lib/Hack';
import { getAllServers as getAllServersUnsafe } from '@/lib/Network' with {type: 'unsafe'};
import { allocateRam, getRamCost, getWorkerServer } from '@/lib/System';
const HACK_THREADS = 5;

export async function main(ns: NS) {
  'use getServerMaxRam';
  'use getServerUsedRam';

  ns.disableLog("ALL");

  const target = await allocateRam(ns, {
    ram: getRamCost(ns, [
      'scan',
      'formulas.hacking.hackPercent',
      'formulas.hacking.hackTime',
      'formulas.hacking.growThreads',
      'formulas.hacking.growTime',
      'formulas.hacking.weakenTime',
      'getPlayer',
      'getServer',
      'weakenAnalyze',
      'hackAnalyzeSecurity',
      'growthAnalyzeSecurity',
    ])
  }, ns => {
    const servers = getAllServersUnsafe(ns)
      .map(s => ns.getServer(s))
      .filter(s => !s.purchasedByPlayer && s.hasAdminRights && s.moneyMax != 0);

    const player = ns.getPlayer();
    const hosts = getAllServersUnsafe(ns).map(s => ns.getServer(s)).filter(s => !s.purchasedByPlayer && s.hostname != 'home').map(s => s.hostname);

    const scores = servers.map(server => {
      const model = { ...server };

      model.hackDifficulty = model.minDifficulty;
      model.moneyAvailable = model.moneyMax;

      const stolen = Math.floor(ns.formulas.hacking.hackPercent(server, player) * HACK_THREADS * model.moneyAvailable!);
      // const hackTime = ns.formulas.hacking.hackTime(model, player);

      model.moneyAvailable! -= stolen;

      const growThreads = ns.formulas.hacking.growThreads(model, player, model.moneyMax!);
      // const growTime = ns.formulas.hacking.growTime(model, player);


      model.hackDifficulty! -= ns.hackAnalyzeSecurity(1);
      model.hackDifficulty! -= ns.growthAnalyzeSecurity(growThreads);
      const securityDifference = model.minDifficulty! - model.hackDifficulty!;
      const weakenTime = ns.formulas.hacking.weakenTime(server, player);
      const weakenThreads = Math.max(Math.ceil(securityDifference / ns.weakenAnalyze(1)), 1);

      const ram = [
        HACK_THREADS * getRamCost(ns, ['hack']),
        growThreads * getRamCost(ns, ['grow']),
        weakenThreads * getRamCost(ns, ['weaken'])
      ] as const;

      return {
        ram,
        stolen,
        batchTime: weakenTime,
        score: stolen / ram.reduce((a, b) => a + b) / weakenTime,
        hostname: server.hostname,
        hackThreads: HACK_THREADS,
        growThreads,
        weakenThreads,
        hosts
      };
    });
    // return scores.toSorted((a, b) => b.score - a.score)[0];
    return scores.toSorted((a, b) => a.batchTime - b.batchTime)[0];
  });

  await fullyWeakenServer(ns, target.hostname);
  await fullyGrowServer(ns, target.hostname);
  await fullyWeakenServer(ns, target.hostname);

  let failed = false;
  const queue: Promise<any>[] = [];
  console.log(target); 

  while (!failed) {
    // for (let i = 0; i < 3; i++) {
    const hostChoices = target.hosts.map(hostname => ({
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

type BatchInfo = {
  hostname: string;
  hackThreads: number;
  growThreads: number;
  weakenThreads: number;
};

async function batch(ns: NS, batch: BatchInfo, [hackHost, growHost, weakenHost]: readonly [string, string, string]) {
  const hackOffset = ns.getWeakenTime(batch.hostname) - ns.getHackTime(batch.hostname);
  const growOffset = ns.getWeakenTime(batch.hostname) - ns.getGrowTime(batch.hostname);

  const order: number[] = [];

  await Promise.all([
    allocateRam(ns, { host: hackHost, ram: getRamCost(ns, ['hack']), threads: batch.hackThreads }, async ns => (await ns.hack(batch.hostname, { additionalMsec: hackOffset }), order.push(0))),
    allocateRam(ns, { host: growHost, ram: getRamCost(ns, ['grow']), threads: batch.growThreads }, async ns => (await ns.grow(batch.hostname, { additionalMsec: growOffset }), order.push(1))),
    allocateRam(ns, { host: weakenHost, ram: getRamCost(ns, ['weaken']), threads: batch.weakenThreads }, async ns => (await ns.weaken(batch.hostname), order.push(2))),
  ]);

  if (!order.every((o, i) => o == i)) {
    throw new Error("Batch failed");
  }

}

