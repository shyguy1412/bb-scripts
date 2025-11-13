import { Taskbar } from './components/Taskbar';
import { Desktop } from './components/Desktop';
import component_style from './style/DesktopEnviroment.css' with {'type': 'css'};
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';
import { adoptStyle } from '@/lib/BitburnerDOM';

export function DesktopEnviroment() {

  const ns = useContext(NetscriptContext);

  const theme = Object.fromEntries(
    Object.entries(ns.ui.getTheme()).map(([k, v]) => (['--' + k, v]))
  );

  adoptStyle(ns, component_style);

  return <>
    <div className='desktop-enviroment' style={theme}>
      <Desktop></Desktop>
      <Taskbar></Taskbar>
    </div>
  </>;
}