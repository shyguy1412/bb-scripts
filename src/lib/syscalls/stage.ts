export function createStageContext(ns: NS, init?: () => void) {
    const parent = ns.self().parent;

    if (parent == 0) {
        init?.();
        ns.writePort(ns.pid, { stage: 0, depth: 0 });
    }

    let currentStage = 0;
    let currentDepth = 0;

    const state = ns.readPort(ns.self().parent || ns.pid);

    return (async <T>(cb: () => T | Promise<T>) => {
        if (currentStage != state.stage && currentDepth == state.depth) {
            const data = state.data[currentStage];
            currentStage++;
            return data as T;
        }

        currentDepth++;
        const result = await cb();
        currentDepth--;

        ns.writePort(ns.pid, {
            stage: currentStage + 1,
            depth: currentDepth,
            data: (state.data ?? []).concat([result]),
        });

        ns.ramOverride(ns.ramOverride() + 2);
        ns['spawn'](ns.self().filename, { spawnDelay: 0 });

        ns.exit(); //explicit exit to actually stop execution of code
    });
}
