import { Hooks, useReactBurner } from '@/lib/stage';

export async function main(ns: NS) {
    const hooks = useReactBurner(ns);

    const [servers, setServers, serversId] = hooks.useState([] as string[]);

    hooks.useEffect(() => hooks.useDynamic('tprint', [servers]), [
        serversId,
    ]);

    hooks.useEffect(() => setServers(getAllServers(hooks)), []);

    hooks.useDynamic('tprint', [hooks.useDynamic('self').dynamicRamUsage]);
}

function getAllServers(hooks: Hooks): string[] {
    const servers = hooks.useDynamic('scan', ['home']);

    for (const server of servers) {
        const newServers = hooks.useDynamic('scan', [server]).slice(1);

        servers.push(...newServers.filter((s) => !servers.includes(s)));
    }
    return servers;
}
