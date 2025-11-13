import { DesktopEnviroment } from './DesktopEnviroment';
import { NetscriptContext, TerminateContext } from '@/lib/Context';

import { createPortal } from 'react-dom';
import React, { createContext } from 'react';
import { enable_hot_reload } from '@/lib/syscalls/hot_reload';
import global_style from './style/global.css' with {'type': 'css'};
import { adoptStyle } from '@/lib/BitburnerDOM';

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
export const PLASMA_CONFIG_FILE = '/etc/plasma/plasmaconf.json';

export async function Plasma(ns: NS) {

  enable_hot_reload(ns);
  adoptStyle(ns, global_style);

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

  if (!config["terminal"] || !ns.fileExists(config["terminal"]!)) {
    ns.toast("No terminal app was set!", 'error');
  }

  if (!config['explorer'] || !ns.fileExists(config['explorer']!)) {
    ns.toast("No file explorer app was set!", 'error');
  }


  return new Promise<void>(resolve => {
    const el = [...document.querySelector('#root')!.children]
      .filter(el => !el.classList.contains('react-draggable') && el.id != '#unclickable')[0];

    ns.tprintRaw(<>
      {createPortal(
        <NetscriptContext.Provider value={ns}>
          <TerminateContext.Provider value={resolve}>
            <DesktopEnviroment></DesktopEnviroment>
          </TerminateContext.Provider>
        </NetscriptContext.Provider>
        , el)}
    </>
    );
  });

}