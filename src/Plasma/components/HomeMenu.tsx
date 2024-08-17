import { ConfigContext } from '@/Plasma/main';
import { CleanupContext, NetscriptContext, TerminateContext } from '@/lib/Context';
import { sleep } from '@/lib/System';
import { Terminal } from '@/lib/Terminal';
import { faJs } from '@fortawesome/free-brands-svg-icons';
import { faCog, faFolderOpen, faPowerOff, faRotate, faTerminal } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';

type Props = {

};

export function HomeMenu({ }: Props) {
  'use run';
  const terminate = useContext(TerminateContext);
  const config = useContext(ConfigContext);
  const ns = useContext(NetscriptContext);
  const addCleanup = useContext(CleanupContext);

  const entries = [...config.get('homeapps')?.map(a => ({ path: a, icon: faJs })) ?? []];

  if (config.get('explorer')) {
    entries.push({
      path: config.get('explorer')!,
      icon: faFolderOpen
    });
  }

  if (config.get('terminal')) {
    entries.push({
      path: config.get('terminal')!,
      icon: faTerminal
    });
  }

  return <div className='homemenu plasma-box-inline'>

    {entries.map(({ path, icon }) =>(console.log(),
      <span className='plasma-button plasma-box-top' onClick={() => { ns.run(path); }}>
        <FontAwesomeIcon icon={icon}></FontAwesomeIcon>
        <span className='plasma-center'>{path.split(/(.*)\//).at(-1)!.split(/(.*)\./, 2)[1]}</span>
      </span>)
    )}


    <span className='plasma-button plasma-box-top' onClick={() => {
      const term = new Terminal(ns);
      term.exec('home;nano .plasmaconf.txt');
      term.cleanup();
    }}>
      <FontAwesomeIcon icon={faCog}></FontAwesomeIcon>
      <span className='plasma-center'>Settings</span>
    </span>

    <span className='plasma-button plasma-box-top' onClick={async () => {
      terminate();
      addCleanup(() => {
        ns.run(ns.getScriptName());
      });
    }}>
      <FontAwesomeIcon icon={faRotate}></FontAwesomeIcon>
      <span className='plasma-center'>Reboot</span>
    </span>

    <span className='plasma-button plasma-box-top' onClick={() => terminate()}>
      <FontAwesomeIcon icon={faPowerOff}></FontAwesomeIcon>
      <span className='plasma-center'>Shutdown</span>
    </span>
  </div>;
}