import { Taskbar } from '@/bb-plasma/components/Taskbar';
import { mapObject } from '@/bb-plasma/lib/MapObject';
import { WindowManagerReducer, WindowManager } from '@/bb-plasma/lib/WindowManager';
import { Desktop } from '@/bb-plasma/components/Desktop';
import { Program } from '@/bb-plasma/lib/Program';
import Style from '@/bb-plasma/style/DesktopEnviroment.css';
import GlobalStyle from '@/bb-plasma/style/global.css';
import React, { createContext, useReducer, useEffect } from 'react';

type Props = {
  ns: NS;
  terminate: () => void;
  reboot: () => void;
};

export const TerminateContext = createContext<Partial<{ terminate: Props['terminate']; }>>({});
export const RebootContext = createContext({ reboot: () => { } });
export const WindowManagerContext = createContext<WindowManager>(null);

export let WindowManagerInstance: WindowManager = [
  { windows: [] },
  () => { throw new Error('WindowManager is not available. Is BBPlasma running?'); }
];

export function DesktopEnviroment({ ns, terminate, reboot }: Props) {

  const windowManager = useReducer(WindowManagerReducer, {
    windows: []
  });

  WindowManagerInstance = windowManager;

  const theme = mapObject(ns.ui.getTheme(), (key, value) => ({
    ['--' + key]: value
  }));

  useEffect(() => {
    return () => {
      //Reset WindowManaferInstance
      WindowManagerInstance = [
        { windows: [] },
        () => { throw new Error('WindowManager is not available. Is BBPlasma running?'); }
      ];
    };
  }, []);

  return <>
    <Style></Style>
    <GlobalStyle></GlobalStyle>
    <div className='desktop-enviroment' style={theme}>
      <WindowManagerContext.Provider value={windowManager}>

        <Desktop></Desktop>

        <RebootContext.Provider value={{ reboot }}>
          <TerminateContext.Provider value={{ terminate }}>
            <Taskbar></Taskbar>
          </TerminateContext.Provider>
        </RebootContext.Provider>

      </WindowManagerContext.Provider>
    </div>
  </>;
}