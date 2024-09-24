import React, { useReducer } from 'react';

type ReactEventType = keyof Omit<React.DOMAttributes<any>, 'children' | 'dangerouslySetInnerHTML'>;
type ReactEvent<K extends ReactEventType> = Parameters<NonNullable<React.DOMAttributes<any>[K]>>[0];

export type FapEvents<T> = {
  [key in ReactEventType]: (cb: (el: FapElement<T>, ev: ReactEvent<key>) => void) => FapElement<T>;
};

type AttributeSetter<T> = (attr: string | number | boolean) => FapElement<T>;
export type Attributes<T> = HTMLAttributes<T> & AriaAttributes<T> & DataAttributes<T>;

type HTMLAttributeKeys<T> = Capitalize<keyof Omit<React.AllHTMLAttributes<T>, keyof FapEvents<T> | keyof React.AriaAttributes | 'className'> | 'class'>;
export type HTMLAttributes<T> = Required<{
  [key in HTMLAttributeKeys<T>]: AttributeSetter<T>;
}>;

type AriaAttributeKeys = keyof React.AriaAttributes extends `aria-${infer A}` ? `Aria${Capitalize<A>}` : never;
export type AriaAttributes<T> = {
  [key in AriaAttributeKeys]: AttributeSetter<T>
};

export type DataAttributes<T> = {
  [key in `Data${Capitalize<string>}`]: AttributeSetter<T>
};

export type FapContent = React.ReactNode | React.FunctionComponent<any> | FapContent[];

export type FapElement<T> = React.JSX.Element & FapEvents<T> & FapModifier<T> & Attributes<T>;
export type FapComponent<T = any> = (content?: FapContent) => FapElement<T>;

export type FapComponents = {
  [key in Capitalize<keyof HTMLElementTagNameMap>]: FapComponent<HTMLElementTagNameMap[Uncapitalize<key>]>
} & {
  'Fragment': (content?: FapContent) => React.JSX.Element;
  'RawElement': <T extends Element>(el: T) => React.JSX.Element;
};

export type FapModifier<T> = {
  Content: (newContent: FapContent) => FapElement<T>;
  Style: (style: React.CSSProperties) => FapElement<T>;
};

export type FapState<T> = {
  events: { [key in ReactEventType]?: (ev: ReactEvent<key>) => void },
  attributes: { [key in Uncapitalize<keyof Attributes<T>>]?: string };
  content: FapContent,
  style: React.CSSProperties,
};

export type Fapped<T> = {
  state: T;
  bind: () => T;
  render?: () => void;
};

export function useFap<T extends {}>(state: T): Fapped<T> {
  const fap: Fapped<T> = {
    state: new Proxy(state, {
      set(target, p, newValue) {
        const set = Reflect.set(target, p, newValue)
        if (fap.render && p != 'render') fap.render();
        return set;
      },
    }),
    bind: () => (fap.render = useReducer(_ => ({}), {})[1], fap.state)
  };
  return fap;
};

function isEventHandler(p: string): p is ReactEventType {
  return /^on[A-Z]/.test(p);
}

function isModifierProp(p: string): p is keyof FapModifier<any> {
  return p == 'Content' || p == 'Style';
}

export const recursivePrepareContent = (content: FapContent): React.ReactNode => {
  if (typeof content == 'function') {
    return React.createElement(content);
  }
  if (typeof content == 'object' && content instanceof Array) {
    return content.map(recursivePrepareContent);
  }
  return content;
};

export const FapComponents = new Proxy({} as FapComponents, {
  get(_, p) {
    if (typeof p == 'symbol') return undefined;
    if (!/^[A-Z]/.test(p)) return undefined;

    if (p == 'Fragment')
      return (content?: React.ReactNode) => React.createElement(
        React.Fragment,
        {},
        recursivePrepareContent(content)
      );

    if (p == 'RawElement') return (child: Element) =>
      React.createElement(
        'span',
        { ref: (el: HTMLSpanElement) => el?.parentElement?.replaceChild(child, el) }
      );

    return (content?: FapContent) => {

      const { state, ...fap } = useFap<FapState<any>>({
        events: {},
        attributes: {},
        content,
        style: {}
      });

      const Component = ({ _ref }: any) => {
        const {
          events, attributes, content, style
        } = fap.bind();

        return React.createElement(
          p.toLocaleLowerCase(),
          { ...events, ...attributes, style, ref: _ref },
          recursivePrepareContent(content)
        );
      };

      return new Proxy(
        React.createElement(Component),
        {
          get(target, p: keyof FapElement<any>, receiver) {
            if (typeof p == 'symbol') {
              return Reflect.get(target, p);
            }

            if (isEventHandler(p)) {
              return (cb: any) => (state['events'][p] = cb.bind(undefined, receiver), receiver);
            }

            if (isModifierProp(p)) {
              const modifier = p.toLocaleLowerCase() as Lowercase<typeof p>;
              return (value: any) => (state[modifier] = value, receiver);
            }

            if (p.startsWith('Data') || p.startsWith('Aria')) {
              const attr = p
                .replace(/(?<=.)[A-Z]/g, (s) => `-${s.toLocaleLowerCase()}`)
                .toLocaleLowerCase() as Uncapitalize<keyof Attributes<any>>;
              return (value: any) => (state['attributes'][attr] = value, receiver);
            }

            if (/^[A-Z]/.test(p)) {
              const attribute = p.toLocaleLowerCase() as Uncapitalize<keyof Attributes<any>>;
              return (value: any) => (state['attributes'][attribute] = value, receiver);
            }

            return Reflect.get(target, p);
          },
        }
      );
    };
  },
});

type Bind<T> = Required<{
  [key in keyof T]: [
    () => T[key],
    (value: T[key]) => void
  ]
}>;

export function createBinding<T extends (...args: any[]) => React.ReactNode, A extends Parameters<T>>(component: T, ...args: A): [React.JSX.Element, ...Bind<A>] {

  const { state, bind } = useFap(args);
  const Wrapper = () => component(...bind());

  const binding = args.map((_, i) => [
    () => {
      return state[i];
    },
    (val: any) => {
      state[i] = val;
    }
  ]) as Bind<A>;

  return [React.createElement(Wrapper), ...binding] as const;
}

type RawMethods = { [key: string]: (...args: any[]) => void; };
export type WrappedMethods<C extends () => React.ReactNode, M extends RawMethods> = { [key in keyof M]: (...args: Parameters<M[key]>) => C & WrappedMethods<C, M> };
export type WrappedComponent<C extends () => React.ReactNode, M extends RawMethods> = C & WrappedMethods<C, M>;
export function FapWrap<C extends () => React.ReactNode, M extends RawMethods>(component: C, methods: M) {
  const wrappedMethods = Object.entries(methods).reduce((prev, [key, value]) => {
    prev[key] = (...args:any) => (value(...args), proxy);
    return prev;
  }, {} as any) as WrappedMethods<C, M>;
  const proxy = new Proxy(component, { get: (t, p) => Reflect.get(wrappedMethods, p) ?? Reflect.get(t, p) }) as C & WrappedMethods<C, M>;
  return proxy;
}