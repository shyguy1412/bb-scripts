import { ConfigContext } from '../main';
import { NetscriptContext } from '@/lib/Context';
import { readDir, readFile } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {
  'use run';

  const ns = useContext(NetscriptContext);
  const config = useContext(ConfigContext);

  const [_, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    const timeout = setTimeout(() => reload(!_), 500); //just swapping between true/false
    return () => clearTimeout(timeout);
  });

  const explorerScript = config.get('explorer');

  return <div className='plasma-desktop'>
    <DoubleClickFileContext.Provider value={(e, { type, name }) => {
      switch (type) {
        case 'js':
          ns.run(name);
          break;
        case 'folder':
          explorerScript && ns.run(explorerScript, undefined, `home/${name}`);
          break;
        case 'txt':
          ns.alert(readFile(ns, `home/${name}`));
          break;
        case 'exe':
          ns.toast('.exe files can only be run from the terminal', 'error');
          break;
        default:
          ns.toast(`This filetype is not supported (${type})`, 'error');
          break;
      }
    }}>

      <FileGrid path={'home'} files={readDir(ns, 'home')} ></FileGrid>
    </DoubleClickFileContext.Provider>
  </div>;
}