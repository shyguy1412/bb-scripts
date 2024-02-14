import { NetscriptContext } from '@/lib/Context';
import { readDir, readFile } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {

  const ns = useContext(NetscriptContext);

  const [_, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    const timeout = setTimeout(() => reload(!_), 500); //just swapping between true/false
    return () => clearTimeout(timeout);
  });

  return <div className='plasma-desktop'>
    <DoubleClickFileContext.Provider value={(e, { type, name }) => {
      switch (type) {
        case 'js':
          'use exec';
          const [server, script] = `home/${name}`.split(/\/(.*)/, 2);
          ns.exec(script, server);
          break;
        case 'folder':
          ns.exec('Dolphin.js', 'home', 1, `home/${name}`);
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