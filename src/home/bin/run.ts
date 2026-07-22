import { connect_to_nukeall } from '@/home/bin/nukeall';
import { createDynamicContext, DynamicArguments, DynamicNS, Path } from '@/lib/dynamic';
import { RunOptions } from 'NetscriptDefinitions';

console.log('loaded run script');

export async function main(ns: NS) {
    console.log('worker launched at ' + performance.now());
    const result = await createDynamicContext(ns)(
        ns.args[0] as any,
        ns.args.slice(1).map((v) => JSON.parse(v as string)),
    );

    ns.atExit(() => {
        console.log('worker exit at ' + performance.now());
        ns.writePort(ns.pid, result);
    });
}

export function worker<F extends Path<NS>>(
    ns: DynamicNS,
    host: string,
    opts: number | RunOptions,
    ...[name, args]: DynamicArguments<F>
) {
    return ns('exec', [
        '/bin/run.js',
        host,
        opts,
        name,
        ...(args ?? []).map((v) => JSON.stringify(v)),
    ]);
}
