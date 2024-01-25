import { Taskbar } from '@/bb-plasma/components/Taskbar';
import { mapObject } from '@/bb-plasma/lib/MapObject';
import { Desktop } from '@/bb-plasma/components/Desktop';
import Style from '@/bb-plasma/style/DesktopEnviroment.css';
import React, { createContext, useReducer, useEffect } from 'react';

type Props = {
  ns: NS;
  terminate: () => void;
  reboot: () => void;
};

export const TerminateContext = createContext<Partial<{ terminate: Props['terminate']; }>>({});
export const RebootContext = createContext({ reboot: () => { } });
export const NetscriptContext = createContext<NS>(null);

export function DesktopEnviroment({ ns, terminate, reboot }: Props) {

  const theme = mapObject(ns.ui.getTheme(), (key, value) => ({
    ['--' + key]: value
  }));

  return <>
    <Style></Style>
    <div className='desktop-enviroment' style={theme}>
      <RebootContext.Provider value={{ reboot }}>
        <NetscriptContext.Provider value={ns}>
          <TerminateContext.Provider value={{ terminate }}>
            <Desktop></Desktop>

            <Taskbar></Taskbar>
          </TerminateContext.Provider>
        </NetscriptContext.Provider>
      </RebootContext.Provider>
    </div>
  </>;
}