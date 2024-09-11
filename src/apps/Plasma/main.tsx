import Style from './style/global.css';
import { createPortal } from 'react-dom';
import { DesktopEnviroment } from './DesktopEnviroment';
import React, { createContext } from 'react';
import { CleanupContext, ContextCollection, NetscriptContext, TerminateContext } from '@/lib/Context';

export const CONFIG = '.plasmaconf.txt';

type PlasmaConfig = Partial<{
  homeapps: string[];
  terminal: string;
  explorer: string;
}>;

type ConfigWrapper = {
  __data: PlasmaConfig,
  get<T extends keyof PlasmaConfig>(value: T): PlasmaConfig[T];
  set<T extends keyof PlasmaConfig>(key: T, value: PlasmaConfig[T]): void;
};

// @ts-expect-error
export const ConfigContext = createContext<ConfigWrapper>(null);


export async function Plasma(ns: NS) {

  'use getHostname';
  if (ns.getHostname() != 'home') {
    throw new Error('Plasma can not run on servers');
  }

  if (!ns.fileExists('.plasmaconf.txt')) {
    ns.write('.plasmaconf.txt', JSON.stringify({
      explorer: 'Dolphin.js',
      terminal: 'Konsole.js',
      homeapps: [],
    } as PlasmaConfig));
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

    const el = [...document.querySelector('#root')!.children]
      .filter(el => !el.classList.contains('react-draggable') && el.id != '#unclickable')[0];

    const config: ConfigWrapper = {
      __data: JSON.parse(ns.read(CONFIG)) as PlasmaConfig,
      get: function <T extends keyof PlasmaConfig>(value: T): PlasmaConfig[T] {
        return this.__data[value];
      },
      set: function <T extends keyof PlasmaConfig>(key: T, value: PlasmaConfig[T]): void {
        this.__data[key] = value;
        ns.write(CONFIG, JSON.stringify(this.__data));
      }
    };

    if (!config.get('terminal') || !ns.fileExists(config.get('terminal')!)) {
      ns.toast("No terminal app was set!", 'error');
    }

    if (!config.get('explorer') || !ns.fileExists(config.get('explorer')!)) {
      ns.toast("No file explorer app was set!", 'error');
    }

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
      },
      {
        context: ConfigContext,
        value: config
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