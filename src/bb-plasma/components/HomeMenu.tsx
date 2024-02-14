import { ConfigContext } from '@/bb-plasma/main';
import { NetscriptContext, TerminateContext } from '@/lib/Context';
import { faJs } from '@fortawesome/free-brands-svg-icons';
import { faFolderOpen, faPowerOff, faTerminal } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';

type Props = {

};

export function HomeMenu({ }: Props) {
  const terminate = useContext(TerminateContext);
  const config = useContext(ConfigContext);
  const ns = useContext(NetscriptContext);

  const entries = [...config.get('homeapps')?.map(a => ({ path: a, icon: faJs })) ?? []];

  if (config.get('explorer')) {
    entries.push({
      path: config.get('explorer'),
      icon: faFolderOpen
    });
  }

  if (config.get('terminal')) {
    entries.push({
      path: config.get('terminal'),
      icon: faTerminal
    });
  }

  return <div className='homemenu plasma-box-inline'>

    {entries.map(({ path, icon }) =>
      <span className='plasma-button plasma-box-top' onClick={() => { ns.run(path); }}>
        <FontAwesomeIcon icon={icon}></FontAwesomeIcon>
        <span className='plasma-center'>{path.split(/(.*)\//, 2).at(-1).split(/(.*)\./, 2)[1]}</span>
      </span>
    )}

    <span className='plasma-button plasma-box-top' onClick={() => terminate()}>
      <FontAwesomeIcon icon={faPowerOff}></FontAwesomeIcon>
      <span className='plasma-center'>Shutdown</span>
    </span>
  </div>;
}