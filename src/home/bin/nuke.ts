import { createDynamicContext } from '@/lib/dynamic';
import { createStageContext } from '@/lib/stage';
import { ScriptArg } from 'NetscriptDefinitions';

export async function main(ns: NS) {
    if (!isWorkerArgs(ns.args)) {
        return;
    }
    const [a, b, c] = ns.args;
    type magic<V, T> = {
        [K in keyof V]: T[K];
    };
}

type ScriptArgType = 'string' | 'number' | 'boolean';

type WorkerArg = [string, string, number?];

function is_args(
    args: (ScriptArg | undefined)[],
    types: ScriptArgType[],
): args is WorkerArg {
    return typeof args[0] == 'string' && typeof args[1] == 'string' &&
        typeof args[2] == 'number';
}
