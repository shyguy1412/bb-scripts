import OperatorSymbols from "OperatorSymbols";

console[OperatorSymbols['<<']] = (_, right) => {
  process.stdout.write(right);
  return console;
};

String.prototype[OperatorSymbols['!']] = (v: string) => v.toLocaleUpperCase();
String.prototype[OperatorSymbols['||']] = (l: string, r: string) => {
  const interleave = ([x, ...xs]: string[], ys = []) =>
    x === undefined
      ? ys                             // base: no x
      : [x, ...interleave(ys, xs)];  // inductive: some x

  return interleave([...l], [...r]).join('');
};
String.prototype[OperatorSymbols['||=']] = String.prototype[OperatorSymbols['||']];
String.prototype[OperatorSymbols['++']] = (v, pre) => {
  return v.toLocaleUpperCase();
};

String.prototype[OperatorSymbols['--']] = (v, pre) => {
  return v.toLocaleLowerCase();
};

console << 'Hello World' << '\n' << ':)' << '\n';

console << !(() => !(() => 'hello!!! :)')())() << '\n';

console << ((() => 'hallo')() || 'world') << '\n';

let stringa = 'test';
let stringb = 'string';

stringa ||= stringb;

console << stringa << '\n';

console << ++stringb << '\n';

stringb--;

console << stringb++ << '\n';

console.log(__updateop.temp);
