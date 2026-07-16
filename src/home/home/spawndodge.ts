import { createDynamicContext } from '@/lib/dynamic';
import { createStageContext } from '@/lib/stage';

export async function main(ns: NS) {
    const dynamic = createDynamicContext(ns);
    const stage = createStageContext(ns, () => console.clear());

    const servers = stage(() => {
        const servers = dynamic('scan', ['home']);

        for (const server of servers) {
            const newServers = dynamic('scan', [server]).slice(1);

            servers.push(...newServers.filter((s) => !servers.includes(s)));
        }
        return servers;
    });

    dynamic('tprint', [servers]);

    // return new Promise(() => {});
}
