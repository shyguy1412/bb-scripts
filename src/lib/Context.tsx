import React from "react";
import { createContext } from "react";

//@ts-expect-error context starts as null, is guranteed to not be null wherever its valid
export const NetscriptContext = createContext<NS>(null);
//@ts-expect-error context starts as null, is guranteed to not be null wherever its valid
export const CleanupContext = createContext<(f: () => void) => void>(null);
//@ts-expect-error context starts as null, is guranteed to not be null wherever its valid
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