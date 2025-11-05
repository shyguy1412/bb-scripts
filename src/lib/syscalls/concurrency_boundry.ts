type Concurrent<T> = {
  [_: symbol]: () => T;
};

let _sym: symbol;

export function create_concurrency_boundry<T extends object>(obj: T): Concurrent<T> {
  _sym = _sym ?? Symbol();

  return new Proxy(obj, {

  }) as Concurrent<T>;
}

export async function exit_concurrency_boundry<T>(boundry: Concurrent<T>): Promise<T> {
  if (!_sym) throw new Error("Invalid boundry");
  const f = boundry[Symbol.for("__syscall_concurrency")];
  return f();
} 