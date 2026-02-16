import { Taskbar } from './components/Taskbar';
import { Desktop } from './components/Desktop';
import component_style from './style/DesktopEnviroment.css' with {'type': 'css'};
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';
import { adoptStyle } from '@/lib/BitburnerDOM';

export function DesktopEnviroment() {

  const ns = useContext(NetscriptContext);

  adoptStyle(ns, component_style);

  return <>
    <div className='desktop-enviroment'>
      <Desktop></Desktop>
      <Taskbar></Taskbar>
    </div>
  </>;
}