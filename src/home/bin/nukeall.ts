import { create_service_interface, Request } from '@/lib/service';
import { Hooks, useReactBurner } from '@/lib/stage';
import { getAllServers } from '@/lib/std/net';
import { Server } from 'NetscriptDefinitions';
import __META_FILENAME from 'meta:filename';

type GetNukedRequest = Request<'nuked', undefined>;
type GetRemainingRequest = Request<'remaining', undefined>;

type NukeAllRequest = GetNukedRequest | GetRemainingRequest;

const [
    connect_to_nukeall,
    create_request_channel,
    create_response_channel,
    register_service,
] = create_service_interface<NukeAllRequest, [string[], string]>(
    __META_FILENAME,
);

export { connect_to_nukeall };
export function main(ns: NS) {
    const hooks = useReactBurner(ns);
    const { useState, useEffect, useDynamic, useComputed, useLoop, useRootPid } = hooks;

    useRootPid((port) => register_service(ns, port));
    const requestQueue = useRootPid((port: number) => () =>
        create_request_channel(ns, port)
    );

    const [servers, setServers, serversId] = useState(() =>
        getAllServers(useDynamic).filter((s) => s != 'home')
    );
    const allServers = useComputed(() => servers, []);
    const nukedServers = useComputed(
        () => allServers.filter((s) => !servers.includes(s)),
        [serversId],
    );

    const fullServers = useComputed(
        () => servers.map((s) => useDynamic('getServer', [s])),
        [serversId],
    );
    const notYetRooted = fullServers.filter((s) => !s.hasAdminRights).map((s) =>
        s.hostname
    );

    useEffect(() => {
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

    for (const request of requestQueue()) {
        create_response_channel(ns, request.sender)([
            request.type == 'nuked' ? nukedServers : notYetRooted,
            serversId,
        ]);
    }

    console.log('NUKE TICK');

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
