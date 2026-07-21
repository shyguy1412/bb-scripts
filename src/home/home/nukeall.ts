import { Hooks, useReactBurner } from '@/lib/stage';
import { Server } from 'NetscriptDefinitions';

export function main(ns: NS) {
    const hooks = useReactBurner(ns);
    const { useState, useEffect, useDynamic, useComputed, useLoop } = hooks;

    useEffect(() => console.clear(), []);

    const [servers, setServers, serversId] = useState(() => getAllServers(hooks));

    const fullServers = useComputed(
        () => servers.map((s) => useDynamic('getServer', [s])),
        [serversId],
    );

    useEffect(() => {
        const notYetRooted = fullServers.filter((s) => !s.hasAdminRights).map((s) =>
            s.hostname
        );
        if (notYetRooted.length != servers.length) {
            setServers(notYetRooted);
        }
    }, [serversId]);

    useEffect(() => {
        const newlyRooted = fullServers.filter((s) => !nukeServer(s, hooks)).map((s) =>
            s.hostname
        );
        if (newlyRooted.length != servers.length) {
            setServers(newlyRooted);
        }
    });

    useLoop(1000);
}

function nukeServer(server: Server, hooks: Hooks): boolean {
    const portOpeners = [
        'brutessh',
        'ftpcrack',
        'relaysmtp',
        'httpworm',
        'sqlinject',
    ] as const satisfies (keyof NS)[];

    for (let i = 0; i < (server.numOpenPortsRequired ?? 5); i++) {
        hooks.useDynamic(portOpeners[i], [server.hostname]);
    }

    const rooted = hooks.useDynamic('nuke', [server.hostname]);

    if (rooted) {
        hooks.useDynamic('tprint', ['Rooted: ', server.hostname]);
    }

    return rooted;
}

function getAllServers(hooks: Hooks): string[] {
    const servers = hooks.useDynamic('scan', ['home']);

    for (const server of servers) {
        const newServers = hooks.useDynamic('scan', [server]).slice(1);

        servers.push(...newServers.filter((s) => !servers.includes(s)));
    }
    return servers;
}
