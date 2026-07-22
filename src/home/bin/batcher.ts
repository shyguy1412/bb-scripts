import { useReactBurner } from '@/lib/stage';
import { getAllServers } from '@/lib/std/net';

import { connect_to_nukeall } from '@/home/bin/nukeall';
export async function main(ns: NS) {
    const _target = 'n00dles';
    const hooks = useReactBurner(ns);
    const {
        useState,
        useDynamic,
        useEffect,
        useComputed,
        useRootPid,
        usePoll,
        useSignal,
    } = hooks;

    const [refreshSignal, sendRefreshSignal] = useSignal();

    const [target, setTarget, targetId] = useState(() =>
        useDynamic('getServer', [_target])
    );

    const [write, , read] = useRootPid((port) => connect_to_nukeall(ns, port));

    useEffect(() => write('nuked'), [refreshSignal]);

    const [hosts, hostsId] = usePoll(
        () => read().map((r) => r.data).unwrapOr(undefined),
        [refreshSignal],
    );

    const hostServers = useComputed(
        () => hosts.map((s) => useDynamic('getServer', [s])),
        [hostsId],
    );

    console.log(hostServers.reduce((a, b) => a + b.maxRam, 0));
}
