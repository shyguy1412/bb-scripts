import { getAllServers } from '@/lib/Network' with {type: 'unsafe'};
import { allocateRam, getMaxThreads, getRamCost } from '@/lib/System';
import { Server } from 'NetscriptDefinitions';

export function gainRoot(ns: NS, host: string) {
  'use brutessh';
  'use httpworm';
  'use sqlinject';
  'use relaysmtp';
  'use ftpcrack';
  'use nuke';
  const PortCracker = [
    ns.brutessh,
    ns.httpworm,
    ns.sqlinject,
    ns.relaysmtp,
    ns.ftpcrack,
  ] as const;

  for (const program of PortCracker) {
    try {
      program(host);
    } catch (_) { }
  }

  try {
    ns.nuke(host);
    return true;
  } catch (_) {
    return false;
  }
}

export function getServerStats(ns: NS, server: Server, host: Server) {
  'use weakenAnalyze';
  'use growthAnalyze';
  const serverSecurity = server.hackDifficulty! - server.minDifficulty!;
  const weakenThreads = serverSecurity / ns.weakenAnalyze(1, host.cpuCores);
  const weakenRamCost = getRamCost(ns, ['weaken'], weakenThreads);

  const growthMultiplier = server.moneyMax! / (server.moneyAvailable || 1);
  const growThreads = ns.growthAnalyze(server.hostname, growthMultiplier, host.cpuCores);
  const growRamCost = getRamCost(ns, ['grow'], growThreads);

  return {
    serverSecurity,
    weakenThreads: Math.ceil(weakenThreads),
    weakenRamCost,

    growthMultiplier,
    growThreads: Math.ceil(growThreads),
    growRamCost,
  };
}



export async function getHackTarget(ns: NS) {
  const { server } = await allocateRam(ns, {
    ram: getRamCost(ns, [
      'scan',
      'getServer',
      'getHackingLevel',
      'getHostname'
    ])
  }, (ns) => {

    const server = getAllServers(ns)
      .map(s => ns.getServer(s))
      .filter(s => s.hasAdminRights && !s.purchasedByPlayer)
      .filter(s => s.requiredHackingSkill! < ns.getHackingLevel())
      .reduce((prev, cur) => {
        return cur.moneyMax != 0 &&
          cur.requiredHackingSkill! > prev.requiredHackingSkill! &&
          cur.requiredHackingSkill! < ns.getHackingLevel() / 10 ?
          cur : prev;
      });

    return { server };
  });

  return server;
}

export async function fullyWeakenServer(ns: NS, target: string) {
  while (await allocateRam(ns, {
    ram: getRamCost(ns, [
      'getServer'
    ])
  }, (ns) => {
    const server = ns.getServer(target);
    return server.minDifficulty != server.hackDifficulty;
  })) {

    const { weakenThreads, server } = await allocateRam(ns, {
      ram: getRamCost(ns, [
        'getServer',
        'getHostname',
        'weakenAnalyze'
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      const host = ns.getServer(ns.getHostname());
      const serverSecurity = server.hackDifficulty! - server.minDifficulty!;
      const weakenThreads = Math.ceil(serverSecurity / ns.weakenAnalyze(1, host.cpuCores));
      return {
        server,
        weakenThreads: Math.floor(Math.min(weakenThreads, getMaxThreads(host, getRamCost(ns, ['weaken'])))) || 1
      };
    });


    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'weaken'
      ]),
      threads: weakenThreads
    }, async (ns) => {
      await ns.weaken(server.hostname);
    });
  }
}


export async function fullyGrowServer(ns: NS, target: string) {
  while (await allocateRam(ns, {
    ram: getRamCost(ns, [
      'getServer'
    ])
  }, (ns) => {
    const server = ns.getServer(target);
    return server.moneyMax != server.moneyAvailable;
  })) {

    const { growThreads, server } = await allocateRam(ns, {
      ram: getRamCost(ns, [
        'getServer',
        'getHostname',
        'growthAnalyze'
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      const host = ns.getServer(ns.getHostname());
      const growthMultiplier = server.moneyMax! / (server.moneyAvailable || 1);
      const growThreads = ns.growthAnalyze(server.hostname, 1 + growthMultiplier, host.cpuCores);

      return {
        server,
        growThreads: Math.floor(Math.min(growThreads, getMaxThreads(host, getRamCost(ns, ['grow']))) || 1)
      };
    });


    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'grow'
      ]),
      threads: growThreads
    }, async (ns) => {
      await ns.grow(server.hostname);
    });
  }
}

export async function prepServer(ns: NS, hostname: string, hacknet: string[]) {
  while (true) {
    const [target, hosts] = await allocateRam(ns, {
      ram: getRamCost(ns, [
        'getServer',
      ]),
    }, ns => ([ns.getServer(hostname), hacknet.map(h => ns.getServer(h)).map(s => ({ hostname: s.hostname, freeRam: s.maxRam - s.ramUsed }))] as const)).catch(_ => []);

    if (!target) { await ns.sleep(500); continue; }
    if (target.hackDifficulty == target.minDifficulty && target.moneyAvailable == target.moneyMax) break;


    if (target.hackDifficulty != target.minDifficulty) {
      await Promise.allSettled(hosts.map(async server => {
        const ram = getRamCost(ns, ['weaken']);
        const threads = Math.floor(server.freeRam / ram) || 1;
        await allocateRam(ns, { host: server.hostname, ram, threads }, ns => ns.weaken(target.hostname));
      }));
    } else {
      await Promise.allSettled(hosts.map(async server => {
        const ram = getRamCost(ns, ["grow"]);
        const threads = Math.floor(server.freeRam / ram) || 1;
        await allocateRam(ns, { host: server.hostname, ram, threads }, ns => ns.grow(target.hostname));
      }));
    }

    await ns.sleep(0);
  }
}


export type BatchInfo = {
  server: Server;
  hackThreads: number;
  growThreads: number;
  weakenThreads: number;
};

export async function batch(ns: NS, batch: BatchInfo, [hackHost, growHost, weakenHost]: readonly [string, string, string]) {
  'use getWeakenTime';
  'use getGrowTime';
  'use getHackTime';
  const hackOffset = ns.getWeakenTime(batch.server.hostname) - ns.getHackTime(batch.server.hostname);
  const growOffset = ns.getWeakenTime(batch.server.hostname) - ns.getGrowTime(batch.server.hostname);

  const order: number[] = [];

  await Promise.all([
    allocateRam(ns, { host: hackHost, ram: getRamCost(ns, ['hack']), threads: batch.hackThreads }, async ns => (await ns.hack(batch.server.hostname, { additionalMsec: hackOffset }), order.push(0))),
    allocateRam(ns, { host: growHost, ram: getRamCost(ns, ['grow']), threads: batch.growThreads }, async ns => (await ns.grow(batch.server.hostname, { additionalMsec: growOffset }), order.push(1))),
    allocateRam(ns, { host: weakenHost, ram: getRamCost(ns, ['weaken']), threads: batch.weakenThreads }, async ns => (await ns.weaken(batch.server.hostname), order.push(2))),
  ]);

  if (!order.every((o, i) => o == i)) {
    throw new Error('Batch failed');
  }
}

export async function calculateBatch(ns: NS, target: string, hackThreads: number, queue?: Promise<void>[]): Promise<BatchInfo & { ram: number[]; }> {
  return allocateRam(ns, {
    ram: getRamCost(ns, [
      'getServer',
      'hackAnalyze',
      'growthAnalyze',
      'getServerMaxMoney',
      'hackAnalyzeSecurity',
      'growthAnalyzeSecurity',
      'weakenAnalyze'
    ])
  }, async ns => {
    if (queue) await queue.shift();
    const server = ns.getServer(target);
    const stolen = ns.hackAnalyze(target) * hackThreads;
    const growThreads = Math.ceil(ns.growthAnalyze(target, 1 / (1 - stolen)));
    const hackSecurity = ns.hackAnalyzeSecurity(hackThreads, target);
    const growSecurity = ns.growthAnalyzeSecurity(growThreads, target);
    const weakenThreads = [...new Array(100).fill(1).keys()].find((i) => ns.weakenAnalyze(i) > hackSecurity + growSecurity);
    if (!weakenThreads) throw new Error('Target is too expensive');

    const ram = [
      hackThreads * getRamCost(ns, ['hack']),
      growThreads * getRamCost(ns, ['grow']),
      weakenThreads * getRamCost(ns, ['weaken'])
    ];

    return {
      server,
      hackThreads,
      growThreads,
      weakenThreads,
      ram
    };
  });
}

export async function calculateBatchingTarget(ns: NS, hackThreads: number) {
  return allocateRam(ns, {
    ram: getRamCost(ns, [
      'scan',
      'formulas.hacking.hackPercent',
      'formulas.hacking.growThreads',
      'formulas.hacking.weakenTime',
      'getPlayer',
      'getServer',
      'weakenAnalyze',
      'hackAnalyzeSecurity',
      'growthAnalyzeSecurity',
    ])
  }, async ns => {
    const servers = getAllServers(ns)
      .map(s => ns.getServer(s))
      .filter(s => !s.purchasedByPlayer && s.hasAdminRights && s.moneyMax != 0);

    const scores = servers.map(server => {
      const model = { ...server };
      const player = ns.getPlayer();

      model.hackDifficulty = model.minDifficulty;
      model.moneyAvailable = model.moneyMax;

      const stolen = Math.floor(ns.formulas.hacking.hackPercent(server, player) * hackThreads * model.moneyAvailable!);
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
        hackThreads * getRamCost(ns, ['hack']),
        growThreads * getRamCost(ns, ['grow']),
        weakenThreads * getRamCost(ns, ['weaken'])
      ] as const;

      return {
        ram,
        stolen,
        batchTime: weakenTime,
        score: stolen / ram.reduce((a, b) => a + b) / weakenTime,
        server: server,
        hackThreads,
        growThreads,
        weakenThreads
      };
    });

    return scores.toSorted((a, b) => b.score - a.score)[0];
    // return scores.toSorted((a, b) => a.batchTime - b.batchTime)[0];
  });
}

export async function hackServer(ns: NS, target: string, hack_percent: number) {
  while (true) {
    const { server, hackThreads } = await allocateRam(ns, {
      ram: getRamCost(ns, [
        'getServer',
        'getHostname',
        'hackAnalyzeThreads',
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      const host = ns.getServer(ns.getHostname());
      const hackThreads = Math.ceil(ns.hackAnalyzeThreads(server.hostname, server.moneyAvailable! * hack_percent));

      return {
        server,
        hackThreads: Math.floor(Math.min(hackThreads, getMaxThreads(host, getRamCost(ns, ['grow'])))) || 1
      };
    });

    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'hack'
      ]),
      threads: hackThreads
    }, async (ns) => {
      await ns.hack(server.hostname);
    });

    if (await allocateRam(ns, {
      ram: getRamCost(ns, [
        'getServer'
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      return server.moneyMax != server.moneyAvailable;
    })) break;
  }
}