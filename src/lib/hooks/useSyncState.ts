import { useDetachedState } from "@/lib/hooks/useDetachedState";
import { Dispatch, useReducer, useMemo } from "react";

export function useSyncState<T>(val: T): [T, Dispatch<T>] {
  
  const [, rerender] = useReducer(() => ({}), {});
  
  const [[prev], setPrev] = useDetachedState(val);
  const [[state], setStateInternal] = useDetachedState(val);

  const setState = (value: T) => (setStateInternal(value), rerender());

  if (!Object.is(prev, val)) {
    setPrev(val);
    setStateInternal(val);
    return [val, setState];
  }

  return [state, setState];
}