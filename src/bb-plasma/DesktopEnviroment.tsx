import { Taskbar } from '@/bb-plasma/components/Taskbar';
import { mapObject } from '@/bb-plasma/lib/MapObject';
import { Desktop } from '@/bb-plasma/components/Desktop';
import Style from '@/bb-plasma/style/DesktopEnviroment.css';
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';

export function DesktopEnviroment() {

  const ns = useContext(NetscriptContext);

  const theme = mapObject(ns.ui.getTheme(), (key, value) => ({
    ['--' + key]: value
  }));

  return <>
    <Style></Style>
    <div className='desktop-enviroment' style={theme}>
      <Desktop></Desktop>
      <Taskbar></Taskbar>
    </div>
  </>;
}