import React, { useReducer } from "react";

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
};

export type FapState<T> = {
  events: { [key in ReactEventType]?: (ev: ReactEvent<key>) => void },
  attributes: { [key in Uncapitalize<keyof Attributes<T>>]?: string };
  content: React.ReactNode,
  style: React.CSSProperties,
  bind: () => { props: Record<string, any>; content: React.ReactNode; };
  render?: () => void;
};

export function useFap<T>(): FapState<T> {

  const state: FapState<T> = {
    content: undefined,
    events: {},
    style: {},
    attributes: {},
    bind() {

      const [, render] = useReducer(() => {
        return {};
      }, {});

      this.render = render;

      const { content, events, style, attributes } = this;

      return { props: { ...events, ...attributes, style }, content };
    }
  };

  return new Proxy(state, {
    set(target, p, newValue) {
      if (target.render && p != 'render') target.render();
      return Reflect.set(target, p, newValue);
    },
  });

};

export function isEventHandler(p: string): p is ReactEventType {
  return /^on[A-Z]/.test(p);
}

type Keys<T extends Record<string, any>, U extends PropertyKey[] = []> =
  {
    [P in keyof T]: {} extends Omit<T, P> ? [P] : [P, ...Keys<Omit<T, P>, U>]
  }[keyof T];

export function isModifierProp(p: string): p is keyof FapModifier<any> {
  const modifierProps: Keys<FapModifier<any>> = ["Style", "Content"];

  return modifierProps.includes(p as any);
}

type FapComponents = {
  [key in Capitalize<keyof HTMLElementTagNameMap>]: (content?: React.ReactNode) => FapElement<HTMLElementTagNameMap[Uncapitalize<key>]>
};

export const FapComponents = new Proxy({} as FapComponents, {
  get(_, p) {
    if (typeof p == 'symbol') return undefined;
    if (!/^[A-Z]/.test(p)) return undefined;

    return (content?: React.ReactNode) => {

      const state = useFap();

      state.content = content;

      const Component = () => {
        const {
          props, content
        } = state.bind();

        return React.createElement(p.toLocaleLowerCase(), props, content);
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
              return (value: any) => (state[p.toLocaleLowerCase() as keyof FapState<any>] = value, receiver);
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
