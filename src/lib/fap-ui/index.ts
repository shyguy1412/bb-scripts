import React, { useReducer } from "react";

type ReactEventType = keyof Omit<React.DOMAttributes<any>, 'children' | 'dangerouslySetInnerHTML'>;
type ReactEvent<K extends ReactEventType> = Parameters<NonNullable<React.DOMAttributes<any>[K]>>[0];

export type FapElement = React.JSX.Element & FapEvents & FapModifier & Attributes;

export type FapEvents = {
  [key in ReactEventType]: (cb: (el: FapElement, ev: ReactEvent<key>) => void) => FapElement;
};

export type Attributes = Required<{
  [key in Capitalize<keyof Omit<React.HTMLAttributes<any>, keyof FapEvents>>]: (attr: string | number) => FapElement;
}>;

export type FapModifier = {
  Content: (newContent: React.ReactNode) => FapElement;
  Style: (style: React.CSSProperties) => FapElement;
};

export type FapState = {
  events: { [key in ReactEventType]?: (ev: ReactEvent<key>) => void },
  attributes: { [key in Uncapitalize<keyof Attributes>]?: string };
  content: React.ReactNode,
  style: React.CSSProperties,
  bind: () => { props: Record<string, any>; content: React.ReactNode; };
  render?: () => void;
};

export function useFap(): FapState {

  const state: FapState = {
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

export function isModifierProp(p: string): p is keyof FapModifier {
  const modifierProps: Keys<FapModifier> = ["Style", "Content"];

  return modifierProps.includes(p as any);
}

export const FapComponents = new Proxy({} as Record<Capitalize<keyof HTMLElementTagNameMap>, (content?: React.ReactNode) => FapElement>, {
  get(_, p) {
    if (typeof p == 'symbol') return undefined;
    if (!/^[A-Z]/.test(p)) return undefined;

    return (content?: React.ReactNode): FapElement => {

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
          get(target, p: keyof FapElement, receiver) {
            if (typeof p == 'symbol') {
              return Reflect.get(target, p);
            }

            if (isEventHandler(p)) {
              return (cb: any) => (state['events'][p] = cb.bind(undefined, receiver), receiver);
            }

            if (isModifierProp(p)) {
              console.log({ mod: p });

              return (value: any) => (state[p.toLocaleLowerCase() as keyof FapState] = value, receiver);
            }

            if (/^[A-Z]/.test(p)) {
              console.log({ attr: p });
              return (value: any) => (state['attributes'][p.toLocaleLowerCase() as Uncapitalize<keyof Attributes>] = value, receiver);
            }

            return Reflect.get(target, p);
          },
        }
      ) as any;
    };
  },
});
