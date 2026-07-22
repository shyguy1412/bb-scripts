import * as hooks from './hooks';

export type Hooks = {
    [K in keyof typeof hooks]: ReturnType<typeof hooks[K]>;
};

export function createContext() {
}

export function useReactBurner(ns: NS): Hooks {
    const parent = ns.self().parent;

    const global = parent ? ns.readPort(parent) : {
        rootPid: ns.pid,
        cur: 0,
        states: [],
    } as hooks.GlobalState;

    ns.writePort(ns.pid, global);

    //reserve the parent pid for services
    if (!parent) {
        ns.ramOverride(ns.ramOverride() + 2);
        ns['spawn'](ns.self().filename, { spawnDelay: 0 });
        ns.exit();
    }

    const boundHooks = Object.entries(hooks).map(([k, v]) => [k, v(ns)] as const);
    return Object.fromEntries(boundHooks) as Hooks;
}
