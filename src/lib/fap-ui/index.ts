import type _React from 'NetscriptDefinitions';
import React, { useReducer } from 'react';

type ReactEventType = keyof Omit<React.DOMAttributes<any>, 'children' | 'dangerouslySetInnerHTML'>;
type ReactEvent<K extends ReactEventType> = Parameters<NonNullable<React.DOMAttributes<any>[K]>>[0];

export type FapElement<T> = React.JSX.Element & FapEvents<T> & FapModifier<T> & Attributes<T>;

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

export type FapModifier<T> = {
  Content: (newContent: React.ReactNode) => FapElement<T>;
  Style: (style: React.CSSProperties) => FapElement<T>;
  Update: () => FapElement<T>;
};

export type FapState<T> = {
  events: { [key in ReactEventType]?: (ev: ReactEvent<key>) => void },
  attributes: { [key in Uncapitalize<keyof Attributes<T>>]?: string };
  content: React.ReactNode,
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
        if (fap.render && p != 'render') fap.render();
        return Reflect.set(target, p, newValue);
      },
    }),
    bind: () => {

      const [, render] = useReducer(() => {
        return {};
      }, {});

      fap.render = render;

      return fap.state;
    }
  };

  return fap;

};

export function isEventHandler(p: string): p is ReactEventType {
  return /^on[A-Z]/.test(p);
}

type Keys<T extends Record<string, any>, U extends PropertyKey[] = []> =
  {
    [P in keyof T]: {} extends Omit<T, P> ? [P] : [P, ...Keys<Omit<T, P>, U>]
  }[keyof T];

export function isModifierProp(p: string): p is keyof FapModifier<any> {
  const modifierProps: Keys<FapModifier<any>> = ['Style', 'Content', 'Update'];

  return modifierProps.includes(p as any);
}

export type FapComponent<T> = (content?: React.ReactNode) => FapElement<T>;

type FapComponents = {
  [key in Capitalize<keyof HTMLElementTagNameMap>]: FapComponent<HTMLElementTagNameMap[Uncapitalize<key>]>
};

export const FapComponents = new Proxy({} as FapComponents, {
  get(_, p) {
    if (typeof p == 'symbol') return undefined;
    if (!/^[A-Z]/.test(p)) return undefined;

    return (content?: React.ReactNode) => {

      const { state, ...fap } = useFap<FapState<any>>({
        events: {},
        attributes: {},
        content: undefined,
        style: {}
      });

      state.content = content;

      const Component = () => {
        const {
          events, attributes, content, style
        } = fap.bind();

        return React.createElement(p.toLocaleLowerCase(), { ...events, ...attributes, style }, content);
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

            if (isModifierProp(p) && p == 'Update') {
              if (fap.render) fap.render();
              return receiver;
            }

            if (isModifierProp(p)) {
              return (value: any) => (state[p.toLocaleLowerCase() as Lowercase<typeof p>] = value, receiver);
            }

            if (p.startsWith('Data') || p.startsWith('Aria')) {
              const attr = p.replace(/(?<=.)[A-Z]/g, (s) => `-${s.toLocaleLowerCase()}`).toLocaleLowerCase() as Uncapitalize<keyof Attributes<any>>;
              return (value: any) => (state['attributes'][attr] = value, receiver);
            }

            if (/^[A-Z]/.test(p)) {
              return (value: any) => (state['attributes'][p.toLocaleLowerCase() as Uncapitalize<keyof Attributes<any>>] = value, receiver);
            }

            return Reflect.get(target, p);
          },
        }
      ) as any;
    };
  },
});


type Bind<T> = Required<{
  [key in keyof T]: [
    () => T[key],
    (value: T[key]) => void
  ]
}>;

export function createBinding<T extends (...args: any[]) => R, R extends FapElement<any>>(component: T, ...args: Parameters<T>) {

  const store = [...args];

  const binding = new Proxy(args as any, {
    get(target, p) {
      if (typeof p == 'symbol') {
        return Reflect.get(target, p);
      }

      if (isNaN(p as any)) {
        return Reflect.get(target, p);
      }

      return [
        () => {
          return store[p as any];
        },
        (val: any) => {
          store[p as any] = val;
          if (state.render) state.render();
        }
      ];
    },
  }) as Bind<Parameters<T>>;

  const state = useFap(store);

  const Wrapper = () => {

    state.bind();

    return component(...store);
  };

  return [React.createElement(Wrapper) as _React.ReactNode, ...binding] as const;
}