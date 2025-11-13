import style from '../style/Taskbar.css' with {'type': 'css'};
import { HomeButton } from '../components/HomeButton';
import React, { useState, useEffect } from 'react';
import { adoptStyle } from '@/lib/hooks/useStyle';

export function Taskbar() {

  const [clock, setClock] = useState(Date.now());

  adoptStyle(style);

  useEffect(() => {
    const interval = setInterval(() => {
      setClock(Date.now());
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  return <>
    <div className='taskbar'>
      <HomeButton></HomeButton>

      <div className='taskbar-windows'>
      </div>

      <span className='taskbar-date plasma-box-inline'>
        <div>{new Date(clock).toLocaleTimeString().slice(0, -3)}</div>
        <div>{new Date(clock).toLocaleDateString()}</div>
      </span>
    </div>
  </>;
}