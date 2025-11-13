import { createContext } from "react";

//@ts-expect-error
export const NetscriptContext = createContext<NS>(null);
//@ts-expect-error
export const TerminateContext = createContext<() => void>(null);
//@ts-expect-error
export const TailRootContext = createContext<HTMLElement>(null);
