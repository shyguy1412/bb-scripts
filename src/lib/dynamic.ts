type NamespaceOf<T> = Omit<
    T,
    {
        [K in keyof T]: T[K] extends Function | number | any[] ? K : never;
    }[keyof T]
>;

type StringKeys<T> = {
    [K in keyof T]: K extends string ? K : never;
}[keyof T];

export type Path<T> = {
    [K in StringKeys<T>]: K extends keyof NamespaceOf<T> ? `${K}.${Path<T[K]>}` :
        K;
}[StringKeys<T>];

type GetByPath<T, P extends Path<T>> = P extends keyof T ? T[P] :
    P extends `${infer Key}.${infer Rest}` ?
        Key extends keyof T ?
            Rest extends Path<T[Key]> ? GetByPath<T[Key], Rest> : never :
        never :
    never;

type AsFunction<T> = T extends (...args: any) => any ? T : never;

export type DynamicArguments<F extends Path<NS>> =
    Parameters<AsFunction<GetByPath<NS, F>>> extends undefined[] ? [
            name: F,
            args?: Parameters<AsFunction<GetByPath<NS, F>>>,
        ] :
        [
            name: F,
            args: Parameters<AsFunction<GetByPath<NS, F>>>,
        ];

export function createDynamicContext(ns: NS): DynamicNS {
    const functionMap: Set<Path<NS>> = new Set();

    return (...[name, args]) => {
        if (!functionMap.has(name)) {
            ns.ramOverride(ns.ramOverride() + ns.getFunctionRamCost(name));
            functionMap.add(name);
        }

        let func = ns as any;
        for (const segment of name.split('.')) {
            func = func[segment];
        }

        return func(...(args ?? []));
    };
}

export type DynamicNS = <F extends Path<NS>>(...args: DynamicArguments<F>) => ReturnType<
    AsFunction<GetByPath<NS, F>>
>;
