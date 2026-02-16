type Foo<T extends Record<string, any>> = {
  [key in keyof T]: number
} & {"bar": string}

const foo:Foo<{"test": number}> = {
  test: 0,
  bar: ""
}

type kebabString = `${string}-${string}`;

type FirstOfKebab<T extends kebabString> = T extends `${infer A}-${string}` ? A : never;

const kebab = "b-a";
const kebabFirst: FirstOfKebab<typeof kebab> = "b";