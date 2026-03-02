const MyTypeA = {
  foo: "",
  bar: 0,
};
type MyTypeA = typeof MyTypeA;

const MyTypeB = {
  foo: "",
  bar: 0,
};
type MyTypeB = typeof MyTypeB;

function isType<T extends object>(v: unknown, type: T): v is T {
  if (typeof v != "object" || !v) {
    return false;
  }

  for (const key in v) {
    const got = v[key as keyof typeof v];
    const wanted = type[key as keyof T];
    if (typeof got != typeof wanted) {
      return false;
    }
    if (typeof got == "object") {
      const isValid = isType(got, wanted as object);
      if (!isValid) return false;
    }
  }

  return true;
}

console.log(isType({
  foo: "",
  bar: 0,
}, MyTypeA));
