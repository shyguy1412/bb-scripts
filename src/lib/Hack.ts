import { getAllServers } from "@/lib/Network" with {type: 'unsafe'};
import { allocateRam, getMaxThreads, getRamCost } from "@/lib/System";
import { Server } from "NetscriptDefinitions";

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
  const serverSecurity = server.hackDifficulty - server.minDifficulty;
  const weakenThreads = serverSecurity / ns.weakenAnalyze(1, host.cpuCores);
  const weakenRamCost = getRamCost(ns, ['weaken'], weakenThreads);

  const growthMultiplier = server.moneyMax / (server.moneyAvailable || 1);
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
      .filter(s => s.requiredHackingSkill < ns.getHackingLevel())
      .reduce((prev, cur) => {
        return cur.moneyMax != 0 &&
          cur.requiredHackingSkill > prev.requiredHackingSkill &&
          cur.requiredHackingSkill < ns.getHackingLevel() / 10 ?
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
        'weakenAnalyze',
        'growthAnalyze',
        'getHostname'
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      const host = ns.getServer(ns.getHostname());
      const { weakenThreads } = getServerStats(ns, server, host);
      return {
        server,
        weakenThreads: Math.min(weakenThreads, getMaxThreads(host, getRamCost(ns, ['weaken'])))
      };
    });


    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'weaken'
      ]),
      threads: weakenThreads
    }, async (ns) => {
      await ns.weaken(server.hostname, { threads: weakenThreads });
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
        'weakenAnalyze',
        'growthAnalyze',
        'getHostname'
      ])
    }, (ns) => {
      const server = ns.getServer(target);
      const host = ns.getServer(ns.getHostname());
      const { growThreads } = getServerStats(ns, server, host);

      return {
        server,
        growThreads: Math.min(growThreads, getMaxThreads(host, getRamCost(ns, ['grow'])))
      };
    });


    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'grow'
      ]),
      threads: growThreads
    }, async (ns) => {
      await ns.grow(server.hostname, { threads: growThreads });
    });
  }
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
      const hackThreads = Math.ceil(ns.hackAnalyzeThreads(server.hostname, server.moneyAvailable * hack_percent));

      return {
        server,
        hackThreads: Math.min(hackThreads, getMaxThreads(host, getRamCost(ns, ['grow'])))
      };
    });

    await allocateRam(ns, {
      ram: getRamCost(ns, [
        'hack'
      ]),
      threads: hackThreads
    }, async (ns) => {
      await ns.hack(server.hostname, { threads: hackThreads });
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