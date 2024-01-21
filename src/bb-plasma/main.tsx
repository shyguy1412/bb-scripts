import { render, unmountComponentAtNode } from 'react-dom';
import { DesktopEnviroment } from './DesktopEnviroment';
import { sleep } from '@/bb-plasma/lib/Sleep';
import React from 'react';



export async function bbplasma(ns: NS) {

  'use getHostname';
  if (ns.getHostname() != 'home') {
    throw new Error('bb-plasma can not run on servers');
  }

  // await sleep(100);

  const overlay = document.createElement('div');
  overlay.style.width = '100vw';
  overlay.style.height = '100vh';
  overlay.style.zIndex = '9999';
  overlay.style.position = 'absolute';
  overlay.style.overflow = 'hidden';
  overlay.style.background = 'black';

  return new Promise<void>(resolve => {
    const reboot = () => {
      'use run';
      ns.run(ns.getScriptName());
      resolve();
    };

    const devTerm = (e: KeyboardEvent) => {
      
      if (e.key == '5') {
        reboot();
      };
      
      if (e.key == 'Escape') {
        resolve();
      }
    };

    window.addEventListener('keydown', devTerm);

    ns.atExit(() => {
      ns.tprint('Terminated');
      removeEventListener('keydown', devTerm);
      unmountComponentAtNode(overlay);
      overlay.remove();
      resolve();
    });

    document.body.prepend(overlay);
    render(<DesktopEnviroment ns={ns} terminate={() => resolve()} reboot={reboot}></DesktopEnviroment>, overlay);
  });

}
