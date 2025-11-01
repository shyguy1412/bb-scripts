import { Taskbar } from './components/Taskbar';
import { mapObject } from './lib/MapObject';
import { Desktop } from './components/Desktop';
// import Style from './style/DesktopEnviroment.css';
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';

export function DesktopEnviroment() {

  const ns = useContext(NetscriptContext);

  const theme = mapObject(ns.ui.getTheme(), (key, value) => ({
    ['--' + key]: value
  }));

  return <>
    <div className='desktop-enviroment' style={theme}>
      <Desktop></Desktop>
      <Taskbar></Taskbar>
    </div>
  </>;
}