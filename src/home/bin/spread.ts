import { connect_to_nukeall } from '@/home/bin/nukeall';
import { createDynamicContext } from '@/lib/dynamic';
import { getAllServers } from '@/lib/std/net';

export async function main(ns: NS) {
    const dynamic = createDynamicContext(ns);

    for (const s of getAllServers(dynamic)) {
        ns.tprint(s);
        ns.scp(ns.args[0] + '', s);
    }
}
