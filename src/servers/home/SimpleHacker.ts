import { fullyGrowServer, fullyWeakenServer, getHackTarget, hackServer } from '@/lib/Hack';

const HACK_PERCENT = 0.8;

export async function main(ns: NS) {
  const server = await getHackTarget(ns);

  while (true) {
    await hackServer(ns, server.hostname, HACK_PERCENT);
    await fullyGrowServer(ns, server.hostname);
    await fullyWeakenServer(ns, server.hostname);
  }
}