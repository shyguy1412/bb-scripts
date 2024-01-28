import Style from './style/global.css';
import { createPortal, render, unmountComponentAtNode } from 'react-dom';
import { DesktopEnviroment } from './DesktopEnviroment';
import { sleep } from '@/lib/System';
import React, { useEffect, useState } from 'react';



export async function bbplasma(ns: NS) {

  'use getHostname';
  if (ns.getHostname() != 'home') {
    throw new Error('bb-plasma can not run on servers');
  }

  // await sleep(100);



  return new Promise<void>(resolve => {
    const reboot = () => {
      'use run';
      ns.run(ns.getScriptName());
      resolve();
    };

    // const devTerm = (e: KeyboardEvent) => {

    //   if (e.key == '5') {
    //     reboot();
    //   };

    //   if (e.key == 'Escape') {
    //     resolve();
    //   }
    // };

    // window.addEventListener('keydown', devTerm);

    ns.atExit(() => {
      ns.tprint('Terminated');
      // removeEventListener('keydown', devTerm);
      'use clearTerminal';
      ns.ui.clearTerminal();
      resolve();
    });

    const el = [...document.querySelector('#root').children]
      .filter(el => !el.classList.contains('react-draggable') && el.id != '#unclickable')[0];

    ns.tprintRaw(<>
      <Style></Style>
      {createPortal(
        <DesktopEnviroment ns={ns} terminate={() => resolve()} reboot={() => reboot()}></DesktopEnviroment>,
        el)}
    </>
    );

    // ns.tprintRaw(<div
    //   className='plasma-wrapper'
    //   id={`plasma-${ns.pid}`}
    //   style={{
    //     width: '100%',
    //     height: '100%',
    //     position: 'absolute',
    //     overflow: 'hidden',
    //     background: 'black',
    //   }}>
    //   <DesktopEnviroment ns={ns} terminate={resolve} reboot={reboot}></DesktopEnviroment>
    // </div>);

    // document.body.prepend(overlay);
    // render(, overlay);
  });

}