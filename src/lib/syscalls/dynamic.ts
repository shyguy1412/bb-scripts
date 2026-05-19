type NamespaceOf<T> = Omit<
    T,
    {
        [K in keyof T]: T[K] extends Function | number | any[] ? K : never;
    }[keyof T]
>;

type StringKeys<T> = {
    [K in keyof T]: K extends string ? K : never;
}[keyof T];

type Path<T> = {
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

function dynamic<F extends Path<NS>>(
    functionMap: Set<Path<NS>>,
    ns: NS,
    name: F,
    args: Parameters<AsFunction<GetByPath<NS, F>>>,
): Promise<
    Awaited<
        ReturnType<
            AsFunction<GetByPath<NS, F>>
        >
    >
> {
    if (!functionMap.has(name)) {
        ns.ramOverride(ns.ramOverride() + ns.getFunctionRamCost(name));
        functionMap.add(name);
    }

    let func = ns as any;
    for (const segment of name.split('.')) {
        func = func[segment];
    }

    return func(...args);
}

export function createDynamicContext(ns: NS) {
    const functionMap: Set<Path<NS>> = new Set();

    return <F extends Path<NS>>(
        name: F,
        args: Parameters<AsFunction<GetByPath<NS, F>>>,
    ) => {
        if (!functionMap.has(name)) {
            ns.ramOverride(ns.ramOverride() + ns.getFunctionRamCost(name));
            functionMap.add(name);
        }

        let func = ns as any;
        for (const segment of name.split('.')) {
            func = func[segment];
        }

        return func(...args) as ReturnType<
            AsFunction<GetByPath<NS, F>>
        >;
    };
}
