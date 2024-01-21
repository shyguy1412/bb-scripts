export function mapObject<T extends { [key: string]: any; }>(obj: T, mapFunction: (key: keyof T, value: any) => ({ [key: string]: any; })) {
  const accumulator: any = {};
  Object.entries(obj).forEach(([key, val]) => Object.assign(accumulator, mapFunction(key, val)));
  return accumulator;
}