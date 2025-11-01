import style from './style/global.css' with {type: 'css'};

import { DesktopEnviroment } from './DesktopEnviroment';
import { CleanupContext, NetscriptContext, TerminateContext } from '@/lib/Context';

import ReactDOM from 'react-dom';
import React, { createContext } from 'react';

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
export const PLASMA_CONFIG_FILE = '.plasmaconf.json';

export async function Plasma(ns: NS) {

  'use getHostname';
  if (ns.getHostname() != 'home') {
    throw new Error('Plasma can not run on servers');
  }

  if (!ns.fileExists(PLASMA_CONFIG_FILE)) {
    ns.write(PLASMA_CONFIG_FILE, JSON.stringify({
      explorer: 'Dolphin.js',
      terminal: 'Konsole.js',
      homeapps: [],
    } as PlasmaConfig));
  }

  const cleanupCallbacks: (() => void)[] = [];
  const addCleanup = (f: () => void) => cleanupCallbacks.push(f);
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
      __data: JSON.parse(ns.read(PLASMA_CONFIG_FILE)) as PlasmaConfig,
      get: function <T extends keyof PlasmaConfig>(value: T): PlasmaConfig[T] {
        return this.__data[value];
      },
      set: function <T extends keyof PlasmaConfig>(key: T, value: PlasmaConfig[T]): void {
        this.__data[key] = value;
        ns.write(PLASMA_CONFIG_FILE, JSON.stringify(this.__data));
      }
    };

    if (!config.get('terminal') || !ns.fileExists(config.get('terminal')!)) {
      ns.toast("No terminal app was set!", 'error');
    }

    if (!config.get('explorer') || !ns.fileExists(config.get('explorer')!)) {
      ns.toast("No file explorer app was set!", 'error');
    }

    ReactDOM.render(
      <NetscriptContext.Provider value={ns}>
        <CleanupContext.Provider value={addCleanup}>
          <TerminateContext.Provider value={resolve}>
            <ConfigContext.Provider value={config}>

              <DesktopEnviroment></DesktopEnviroment>

            </ConfigContext.Provider>
          </TerminateContext.Provider>
        </CleanupContext.Provider>
      </NetscriptContext.Provider>
      , el
    );
  });

}