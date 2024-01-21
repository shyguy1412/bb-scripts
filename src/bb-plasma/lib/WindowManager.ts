import { PlasmaWindow } from '@/bb-plasma/components/PlasmaWindow';

export interface WindowManagerState {
  windows: Parameters<typeof PlasmaWindow>[0][];
}

export type WindowManager = [WindowManagerState, React.Dispatch<WindowManagerDispatch<WindowManagerAction>>];

export type WindowManagerAction = 'CREATE' | 'CLOSE' | 'MINIMIZE' | 'MAXIMISE' | 'FOCUS';

export interface WindowManagerDispatch<T extends WindowManagerAction = WindowManagerAction> {
  action: T;
  window: WindowManagerState['windows'][number];
};

type WindowManagerActionHandlerMap = {
  [key in WindowManagerDispatch['action']]: WindowManagerActionHandler<key>
};

type WindowManagerActionHandler<T extends WindowManagerAction> = (state: WindowManagerState, { action, window }: WindowManagerDispatch<T>) => WindowManagerState;

function getWindow(state: WindowManagerState, window: WindowManagerDispatch['window']) {
  return state.windows.find(w => w.id == window.id);
}

const ActionHandler: WindowManagerActionHandlerMap = {
  CREATE: function (state: WindowManagerState, { action, window }: WindowManagerDispatch<'CREATE'>): WindowManagerState {
    if (getWindow(state, window))
      return;
    state.windows.unshift(window);
    return { ...state };
  },
  CLOSE: function (state: WindowManagerState, { action, window }: WindowManagerDispatch<'CLOSE'>): WindowManagerState {
    return {
      windows: state.windows.filter(w => w.id != window.id)
    };
  },
  MINIMIZE: function (state: WindowManagerState, { action, window }: WindowManagerDispatch<'MINIMIZE'>): WindowManagerState {
    getWindow(state, window).minimized = true;
    state.windows.unshift(state.windows.pop());
    return { ...state };
  },
  MAXIMISE: function (state: WindowManagerState, { action, window }: WindowManagerDispatch<'MAXIMISE'>): WindowManagerState {
    getWindow(state, window).minimized = true;
    return { ...state };
  },
  FOCUS: function (state: WindowManagerState, { action, window }: WindowManagerDispatch<'FOCUS'>): WindowManagerState {
    window.minimized = false;
    return {
      windows: [...state.windows.filter(w => w.id != window.id), window]
    };
  }
};

export function WindowManagerReducer(state: WindowManagerState, dispatch: WindowManagerDispatch) {
  return (ActionHandler[dispatch.action] as WindowManagerActionHandler<typeof dispatch['action']>)(state, dispatch);
}