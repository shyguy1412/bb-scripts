import { Hooks, useReactBurner } from '@/lib/stage';

import { connect_to_nukeall } from '@/home/bin/nukeall';
import { worker } from '@/home/bin/run';
import { Server } from 'NetscriptDefinitions';

export async function main(ns: NS) {
    const hooks = useReactBurner(ns);
    const {
        useDynamic,
        useComputed,
        useRootPid,
        usePoll,
        useSignal,
    } = hooks;

    const [refreshTargetSignal, sendRefreshTargetSignal] = useSignal();
    const [write, , read] = useRootPid((port) => connect_to_nukeall(ns, port));

    useComputed(() => write('nuked'), [refreshTargetSignal]);

    const [hosts, hostsId] = usePoll(
        () => read().map((r) => r.data).unwrapOr(undefined),
        [refreshTargetSignal],
        100,
        15,
    );

    const nukedServers = useComputed(
        () => hosts.map((s) => useDynamic('getServer', [s])),
        [hostsId],
    );

    const hostServers = useComputed(
        () => hosts.map((s) => useDynamic('getServer', [s])).filter((s) => s.maxRam),
        [hostsId],
    );

    const target = useComputed(
        () =>
            nukedServers.sort((a, b) =>
                (b.moneyMax! / b.minDifficulty!) -
                (a.moneyMax! / a.minDifficulty!)
            ).find((s) => s.requiredHackingSkill! < useDynamic('getHackingLevel') / 2),
        [refreshTargetSignal],
    );

    if (!target) {
        throw Error('no suitable target');
    }

    (['weaken', 'grow', 'hack'] as const).map((hgw) =>
        useHgw(
            hooks,
            hgw,
            target,
            hostServers,
            [hostsId, refreshTargetSignal],
            sendRefreshTargetSignal,
        )
    ).map((s) => s());
}

function useHgw(
    hooks: Hooks,
    hgw: 'hack' | 'weaken' | 'grow',
    target: Server,
    hosts: Server[],
    externalDependencies: string[],
    cb?: () => void,
) {
    const {
        useSignal,
        useDynamic,
        useComputed,
        usePoll,
        useLoop,
        useEffect,
    } = hooks;

    const [signal, sendSignal] = useSignal();
    const deps = [
        signal,
        ...externalDependencies,
    ];

    const getTime = ({
        hack: 'getHackTime',
        weaken: 'getWeakenTime',
        grow: 'getGrowTime',
    } as const)[hgw];

    const time = useComputed(() => useDynamic(getTime, [target.hostname]), deps);

    const shouldDo = ({
        hack: target.moneyMax == target.moneyAvailable,
        weaken: target.minDifficulty != target.hackDifficulty,
        grow: target.moneyMax != target.moneyAvailable,
    })[hgw];

    const launchWorker = (s: Server) =>
        worker(
            useDynamic,
            s.hostname,
            { threads: Math.floor(s.maxRam / 1.75), temporary: true },
            hgw,
            [target.hostname],
        );

    const [port] = useComputed(
        () => shouldDo ? hosts.map(launchWorker) : [],
        deps,
    );

    useEffect(() => useLoop(1), deps);

    usePoll(
        () => {
            if (!shouldDo) {
                return true;
            }

            const portHandle = useDynamic('getPortHandle', [port]);
            if (portHandle.empty()) {
                console.log('MISSED RESPAWN WINDOW');
                return;
            }

            return portHandle.read();
        },
        deps,
        Math.ceil(time),
    );

    if (shouldDo) {
        cb?.();
    }

    return () => void (shouldDo && sendSignal());
}
