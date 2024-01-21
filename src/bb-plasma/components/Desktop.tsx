import { PlasmaWindow } from '@/bb-plasma/components/PlasmaWindow';
import { WindowManagerContext } from '@/bb-plasma/DesktopEnviroment';
import React, { useContext } from 'react';

type Props = {

};

export function Desktop({ }: Props) {

  const [{ windows }] = useContext(WindowManagerContext);

  return <div className='plasma-desktop'>
    {windows.map((props) => <PlasmaWindow key={props.id} {...props}></PlasmaWindow>)}
  </div>;
}