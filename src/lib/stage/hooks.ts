import { createDynamicContext } from '@/lib/dynamic';
import { Result } from '@/lib/result';
import { create_service_interface, register_as_service, Request } from '@/lib/service';
import { Dispatch } from 'react';

type UseStateState<T> = {
    value: T;
    id: string;
};

export const useState =
    (ns: NS) => <T>(initial: T | (() => T)): [T, Dispatch<T>, string] => {
        let init = false;
        const { state, commit } = useHookState<UseStateState<T>>(ns, () => (init = true, {
            value: typeof initial == 'function' ? (initial as any)() : initial,
            id: crypto.randomUUID(),
        }));

        if (init && typeof initial == 'function') {
            commit();
        }

        return [state.value, (value) => {
            state.value = value;
            state.id = crypto.randomUUID();
            commit();
        }, state.id];
    };

type MaybeAsyncCallback = (() => Promise<unknown>) | (() => unknown);
type Void<T> = T extends Promise<any> ? Promise<void> : void;
type UseEffectState = undefined | string[];

export const useEffect = (ns: NS) =>
<T extends MaybeAsyncCallback>(
    cb: T,
    dependencies?: string[],
): Void<ReturnType<T>> => {
    const hookState: HookState<UseEffectState> = useHookState<UseEffectState>(
        ns,
        () => undefined,
    );

    if (
        hookState.state &&
        hookState.state.length == dependencies?.length &&
        dependencies.every((id, i) => hookState.state![i] == id)
    ) {
        //@ts-ignore result cached
        return;
    }
    hookState.state = dependencies;
    hookState.patch();

    const result = cb();

    if (!result || typeof result != 'object' || !('then' in result)) {
        //@ts-ignore result is not thenable
        return;
    }

    //@ts-ignore result is thenable
    return new Promise(async (r) => (await result, r()));
};

export const useComputed =
    (ns: NS) => <T>(compute: () => T, dependencies: string[]): T => {
        const hookState: HookState<T | undefined> = useHookState<T | undefined>(
            ns,
            () => undefined,
        );

        let computed = hookState.state;
        useEffect(ns)(() => {
            hookState.state = compute();
            hookState.commit();
        }, dependencies);

        return computed!;
    };

export const useDynamic: (ns: NS) => ReturnType<typeof createDynamicContext> = (ns: NS) =>
    createDynamicContext(ns);

export const useLoop = (ns: NS) => (delay = 0) => {
    useGlobalState(ns, () => {}, true, delay);
};

type PollState<T> = {
    poll: PollReady<T> | PollPending;
    dependencies: undefined | string[];
};
type PollReady<T> = { ready: true; value: T };
type PollPending = { ready: false };

export const usePoll = (ns: NS) =>
<T, E>(
    cb: () => T | undefined,
    dependencies: string[],
    interval = 100,
) => {
    const hookState: HookState<PollState<T>> = useHookState<PollState<T>>(
        ns,
        () => ({ poll: { ready: false }, dependencies: undefined }),
    );

    if (
        hookState.state.poll.ready &&
        hookState.state.dependencies &&
        hookState.state.dependencies.length == dependencies?.length &&
        dependencies.every((id, i) => hookState.state.dependencies![i] == id)
    ) {
        return hookState.state.poll.value;
    }
    hookState.state.dependencies = dependencies;
    hookState.patch();

    console.log({ poll: hookState });

    let result = cb();
    if (result) {
        hookState.state.poll = { ready: true, value: result };
    }
    hookState.commit(interval);
};

export const useSignal = (ns: NS) => () => {
    const [flag, setFlag, signal] = useState(ns)(false);

    return [signal, () => setFlag(!flag)] as const;
};

type HookState<T> = {
    state: T;
    patch(): void;
    commit(delay?: number): never;
};

export type GlobalState = {
    rootPid: number;
    cur: number;
    states: any[];
};

export const useRootPid = (ns: NS) => <T>(cb: (port: number) => T): T => {
    const rootPid = useGlobalState(ns, (global) => global.rootPid);
    return cb(rootPid);
};

function useGlobalState<T>(
    ns: NS,
    cb: (gobal: GlobalState) => T,
    commit = false,
    delay = 0,
): T {
    const global = ns.readPort(ns.pid);
    const r = cb(global);

    if (commit) {
        global.cur = 0;
    }

    ns.writePort(ns.pid, global);
    if (commit) {
        ns.ramOverride(ns.ramOverride() + 2);
        ns['spawn'](ns.self().filename, { spawnDelay: delay });
        ns.exit();
    }
    return r;
}

function useHookState<T>(ns: NS, init: () => T): HookState<T> {
    const [id, state] = useGlobalState(ns, (global) => {
        if (global.states.length <= global.cur) {
            global.states.push(init());
        }
        return [global.cur, global.states[global.cur++] as T];
    });

    const updateState = (commit = false, delay = 0) =>
        useGlobalState(
            ns,
            (global) => global.states[id] = hookState.state,
            commit,
            delay,
        );

    const hookState: HookState<T> = {
        state,
        patch: () => updateState(),
        commit: (delay: number) => updateState(true, delay) as never,
    };

    return hookState;
}
