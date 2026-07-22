import { createDynamicContext, DynamicNS } from '@/lib/dynamic';

export async function main(ns: NS) {
    const dynamic = createDynamicContext(ns);
    dynamic('tprint', [path(dynamic, ns.args[0] + '').join('; connect '), '; backdoor']);
}

function path(ns: DynamicNS, target: string): string[] {
    if (target == 'home') {
        return ['home'];
    }
    return [...path(ns, ns('scan', [target])[0]), target];
}
