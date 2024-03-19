import { fullyGrowServer, fullyWeakenServer, hackServer } from '@/lib/Hack';

const HACK_PERCENT = 0.9;

export async function main(ns: NS) {
  // const {hostname} = await getHackTarget(ns);
  const hostname = ns.args[0] as string ?? 'foodnstuff';

  await fullyWeakenServer(ns, hostname);
  await fullyGrowServer(ns, hostname);
  await fullyWeakenServer(ns, hostname);

  while (true) {
    await hackServer(ns, hostname, HACK_PERCENT);
    await fullyGrowServer(ns, hostname);
    await fullyWeakenServer(ns, hostname);
  }
}