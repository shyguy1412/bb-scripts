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
  [K in StringKeys<T>]: K extends keyof NamespaceOf<T> ? `${K}.${Path<T[K]>}`
    : K;
}[StringKeys<T>];

type GetByPath<T, P extends Path<T>> = P extends keyof T ? T[P]
  : P extends `${infer Key}.${infer Rest}`
    ? Key extends keyof T
      ? Rest extends Path<T[Key]> ? GetByPath<T[Key], Rest> : never
    : never
  : never;

type AsFunction<T> = T extends (...args: any) => any ? T : never;

async function dodgeRam<F extends Path<NS>>(
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
  return "" as any;
}

export async function main(ns: NS) {
}
