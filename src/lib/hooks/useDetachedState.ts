import { useMemo } from "react";

export function useDetachedState<T>(val: T) {
  //the useMemo are used like useState that dont trigger a rerender
  return useMemo(() => {
    const state = [val];
    return [state, (val: T) => { state[0] = val; }] as const;
  }, []);
}