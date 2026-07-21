import * as hooks from './hooks';

export type Hooks = {
    [K in keyof typeof hooks]: ReturnType<typeof hooks[K]>;
};

export const ID_SYMBOL = Symbol();
export const STATE_SYMBOL = Symbol();

export function useReactBurner(ns: NS): Hooks {
    const parent = ns.self().parent;

    const global = parent ? ns.readPort(parent) : {
        cur: 0,
        states: [],
    };

    ns.writePort(ns.pid, global);

    const boundHooks = Object.entries(hooks).map(([k, v]) => [k, v(ns)] as const);
    return Object.fromEntries(boundHooks) as Hooks;
}

export function createStageContext(ns: NS, init?: () => void) {
    const parent = ns.self().parent;

    if (parent == 0) {
        init?.();
        ns.writePort(ns.pid, { stage: 0, depth: 0 });
    }

    let currentStage = 0;
    let currentDepth = 0;

    const state = ns.readPort(ns.self().parent || ns.pid);

    return (<T>(cb: () => T) => {
        if (currentStage != state.stage && currentDepth == state.depth) {
            const data = state.data[currentStage];
            currentStage++;
            return data as T;
        }

        currentDepth++;
        const result = cb();

        const restOfFunction = (result: T) => {
            currentDepth--;

            ns.writePort(ns.pid, {
                stage: currentStage + 1,
                depth: currentDepth,
                data: (state.data ?? []).concat([result]),
            });

            ns.ramOverride(ns.ramOverride() + 2);
            ns['spawn'](ns.self().filename, { spawnDelay: 0 });

            ns.exit(); //explicit exit to actually stop execution of code
        };

        if ('then' in (result ?? {})) {
            return new Promise(async (r) => {
                restOfFunction(await result);
            });
        } else {
            restOfFunction(result);
        }
    });
}
