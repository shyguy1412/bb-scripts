import { ConfigContext, PLASMA_CONFIG_FILE } from '../main';
import { CleanupContext, NetscriptContext, TerminateContext } from '@/lib/Context';
import { Terminal } from '@/lib/Terminal';
import React, { useContext } from 'react';
import { FaCog, FaFolderOpen, FaJs, FaPowerOff, FaTerminal } from 'react-icons/fa';
import { FaRotate } from 'react-icons/fa6';

export function HomeMenu() {
  'use run';
  const terminate = useContext(TerminateContext);
  const config = useContext(ConfigContext);
  const ns = useContext(NetscriptContext);
  const addCleanup = useContext(CleanupContext);

  const server = ns.self().server;

  const entries = [...config.get('homeapps')?.map(a => ({ path: a, Icon: FaJs })) ?? []];

  if (config.get('explorer')) {
    entries.push({
      path: config.get('explorer')!,
      Icon: FaFolderOpen
    });
  }

  if (config.get('terminal')) {
    entries.push({
      path: config.get('terminal')!,
      Icon: FaTerminal
    });
  }

  return <div className='homemenu plasma-box-inline'>

    {entries.map(({ path, Icon }) => (console.log(),
      <span className='plasma-button plasma-box-top' onClick={() => { ns.run(path); }}>
        <Icon></Icon>
        <span className='plasma-center'>{path.split(/(.*)\//).at(-1)!.split(/(.*)\./, 2)[1]}</span>
      </span>)
    )}


    <span className='plasma-button plasma-box-top' onClick={() => {
      const term = new Terminal(ns);
      term.exec(`${server};nano ${PLASMA_CONFIG_FILE}`);
      term.cleanup();
    }}>
      <FaCog></FaCog>
      <span className='plasma-center'>Settings</span>
    </span>

    <span className='plasma-button plasma-box-top' onClick={async () => {
      terminate();
      addCleanup(() => {
        ns.run(ns.getScriptName(), 1, "--replace");
      });
    }}>
      <FaRotate></FaRotate>
      <span className='plasma-center'>Reboot</span>
    </span>

    <span className='plasma-button plasma-box-top' onClick={() => terminate()}>
      <FaPowerOff></FaPowerOff>
      <span className='plasma-center'>Shutdown</span>
    </span>
  </div>;
}