export function getPurchasedServers(ns: NS) {
  'use getServer';
  const servers = getAllServers(ns);
  return servers.map(s => ns.getServer(s)).filter(s => s.purchasedByPlayer && s.hostname != 'home');
}

export function getAllServers(ns: NS) {
  'use scan';
  return getAllServersUnsafe(ns);
}

export function getAllServersUnsafe(ns: NS) {
  const servers = ns.scan('home');
  for (const server of servers) {
    servers.push(...ns.scan(server).filter(s => !servers.includes(s)));
  }
  return servers;
}

export function doublePurchasedServerRam(ns: NS, hostname: string) {
  'use getServer';
  'use upgradePurchasedServer';

  const server = ns.getServer(hostname);
  if (!server.purchasedByPlayer) {
    throw new Error('Not a player owned Server');
  }
  return ns.upgradePurchasedServer(server.hostname, server.maxRam * 2);
}

export function maxoutPurchasedServerRam(ns: NS, hostname: string) {
  while (doublePurchasedServerRam(ns, hostname));
}