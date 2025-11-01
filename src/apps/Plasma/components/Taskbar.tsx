// import Style from '../style/Taskbar.css';
import { HomeButton } from '../components/HomeButton';
import React, { useState, useEffect } from 'react';

type Props = {
};

export function Taskbar({ }: Props) {

  const [clock, setClock] = useState(Date.now());

  useEffect(() => {
    const interval = setInterval(() => {
      setClock(Date.now());
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  return <>
    {/* <Style></Style> */}
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