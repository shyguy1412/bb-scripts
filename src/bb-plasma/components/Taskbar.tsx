import Style from '@/bb-plasma/style/Taskbar.css';
import { HomeButton } from '@/bb-plasma/components/HomeButton';
import { WindowManagerContext } from '@/bb-plasma/DesktopEnviroment';
import { TaskbarEntry } from '@/bb-plasma/components/TaskbarEntry';
import React, { useState, useContext, useEffect } from 'react';

type Props = {
};

export function Taskbar({ }: Props) {

  const [clock, setClock] = useState(Date.now());
  const [{ windows }] = useContext(WindowManagerContext);

  useEffect(() => {
    const interval = setInterval(() => {
      setClock(Date.now());
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  return <>
    <Style></Style>
    <div className='taskbar'>
      <HomeButton></HomeButton>

      <div className='taskbar-windows'>
        {[...windows] //sort sorts  in place so we need a new array
          .sort((a, b) => a.id - b.id)
          .map((props) => <TaskbarEntry
            key={props.id}
            {...props}
          ></TaskbarEntry>)
        }
      </div>

      <span className='taskbar-date plasma-box-inline'>
        <div>{new Date(clock).toLocaleTimeString().slice(0, -3)}</div>
        <div>{new Date(clock).toLocaleDateString()}</div>
      </span>
    </div>
  </>;
}