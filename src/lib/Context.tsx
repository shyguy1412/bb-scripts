import React from "react";
import { createContext } from "react";

export const NetscriptContext = createContext<NS>(null);
export const CleanupContext = createContext<(f: () => void) => void>(null);
export const TerminateContext = createContext<() => void>(null);

export type ContextProvider<T> = {
  context: React.Context<T>,
  value: T;
};

type Props = {
  contexts: ContextProvider<any>[];
  children: React.ReactNode;
};

export function ContextCollection({ contexts, children }: Props) {
  return contexts.reduce((previousContextElement, { context: { Provider: CurrentContext }, value }) =>
    <CurrentContext value={value}>{previousContextElement}</CurrentContext>
    , children);
}