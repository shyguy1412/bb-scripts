import Style from './style/global.css';
import { createPortal } from 'react-dom';
import { DesktopEnviroment } from './DesktopEnviroment';
import React from 'react';
import { CleanupContext, ContextCollection, NetscriptContext, TerminateContext } from '@/lib/Context';

export const CONFIG = '.plasmaconf.txt';

type BBConfig = Partial<{
  homeapps: string[];
}>;


export async function bbplasma(ns: NS) {

  'use getHostname';
  if (ns.getHostname() != 'home') {
    throw new Error('bb-plasma can not run on servers');
  }

  if (!ns.fileExists('.plasmaconf.txt')) {
    ns.write('.plasmaconf.txt', JSON.stringify({}));
  }

  const cleanupCallbacks: (() => void)[] = [];
  return new Promise<void>(resolve => {

    ns.atExit(() => {
      ns.tprint('Terminated');
      'use clearTerminal';
      ns.ui.clearTerminal();
      cleanupCallbacks.forEach(c => c());
      resolve();
    });

    const el = [...document.querySelector('#root').children]
      .filter(el => !el.classList.contains('react-draggable') && el.id != '#unclickable')[0];

    const contexts = [
      {
        context: NetscriptContext,
        value: ns
      },
      {
        context: TerminateContext,
        value: resolve
      },
      {
        context: CleanupContext,
        value: (f: () => void) => cleanupCallbacks.push(f)
      }
    ];

    ns.tprintRaw(<>
      <Style></Style>
      {createPortal(
        <ContextCollection contexts={contexts}>
          <DesktopEnviroment></DesktopEnviroment>
        </ContextCollection>
        , el)}
    </>
    );
  });

}