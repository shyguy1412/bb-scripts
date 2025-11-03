import { Taskbar } from './components/Taskbar';
import { Desktop } from './components/Desktop';
import global_style from './style/global.css' with {'type': 'css'};
import component_style from './style/DesktopEnviroment.css' with {'type': 'css'};
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';
import { useStyle } from '@/lib/hooks/useStyle';

export function DesktopEnviroment() {

  const ns = useContext(NetscriptContext);

  const theme = Object.fromEntries(
    Object.entries(ns.ui.getTheme()).map(([k, v]) => (['--' + k, v]))
  );

  useStyle(global_style);
  useStyle(component_style);

  return <>
    <div className='desktop-enviroment' style={theme}>
      <Desktop></Desktop>
      <Taskbar></Taskbar>
    </div>
  </>;
}