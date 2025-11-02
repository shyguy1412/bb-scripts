import { DesktopEnviroment } from './DesktopEnviroment';
import { CleanupContext, NetscriptContext, TerminateContext } from '@/lib/Context';

import { createPortal } from 'react-dom';
import React, { createContext } from 'react';

type PlasmaConfig = Partial<{
  homeapps: string[];
  terminal: string;
  explorer: string;
  desktop: string;
}>;

type ConfigWrapper = {
  get<T extends keyof PlasmaConfig>(value: T): PlasmaConfig[T];
  set<T extends keyof PlasmaConfig>(key: T, value: PlasmaConfig[T]): void;
};

// @ts-expect-error
export const ConfigContext = createContext<ConfigWrapper>(null);
export const PLASMA_CONFIG_FILE = '.plasmaconf.json';

export async function Plasma(ns: NS) {

  // if (ns.getHostname() != 'home') {
  //   throw new Error('Plasma can not run on servers');
  // }

  const self = ns.self();
  const plasmas = ns.ps(self.server).filter(p => p.filename == self.filename);

  if (ns.args.includes("--replace")) {
    plasmas.filter(p => p.pid != ns.pid).map(p => p.pid).forEach(ns.kill);
  }

  if (plasmas.length > 1) {
    throw new Error('Plasma can only run once');
  }

  //This is awfull
  let config: PlasmaConfig = (() => { try { return JSON.parse(ns.read(PLASMA_CONFIG_FILE)); } catch { } })();

  if (!config) {
    config = {
      explorer: '/bin/dolphin.js',
      terminal: '/bin/Konsole.js',
      homeapps: [],
      desktop: "home"
    };
    ns.write(PLASMA_CONFIG_FILE, JSON.stringify(config, null, 2), 'w');
  }

  const cleanupCallbacks: (() => void)[] = [];
  const addCleanup = (f: () => void) => cleanupCallbacks.push(f);
  return new Promise<void>(resolve => {

    ns.atExit(() => {
      ns.ui.clearTerminal();
      cleanupCallbacks.forEach(c => c());
    });

    const el = [...document.querySelector('#root')!.children]
      .filter(el => !el.classList.contains('react-draggable') && el.id != '#unclickable')[0];

    const configWrapper = {
      get: function <T extends keyof PlasmaConfig>(value: T): PlasmaConfig[T] {
        return config[value];
      },
      set: function <T extends keyof PlasmaConfig>(key: T, value: PlasmaConfig[T]): void {
        config[key] = value;
        ns.write(PLASMA_CONFIG_FILE, JSON.stringify(config, null, 2), 'w');
      }
    };

    if (!config["terminal"] || !ns.fileExists(config["terminal"]!)) {
      ns.toast("No terminal app was set!", 'error');
    }

    if (!config['explorer'] || !ns.fileExists(config['explorer']!)) {
      ns.toast("No file explorer app was set!", 'error');
    }

    ns.tprintRaw(<>
      {createPortal(
        <NetscriptContext.Provider value={ns}>
          <CleanupContext.Provider value={addCleanup}>
            <TerminateContext.Provider value={resolve}>
              <ConfigContext.Provider value={configWrapper}>

                <DesktopEnviroment></DesktopEnviroment>

              </ConfigContext.Provider>
            </TerminateContext.Provider>
          </CleanupContext.Provider>
        </NetscriptContext.Provider>
        , el)}
    </>
    );
  });

}