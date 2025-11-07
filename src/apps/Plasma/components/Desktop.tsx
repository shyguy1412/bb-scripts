import { ConfigContext } from '../main';
import { CleanupContext, NetscriptContext } from '@/lib/Context';
import { read_dir, readFile } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {
  'use run';

  const ns = useContext(NetscriptContext);
  const config = useContext(ConfigContext);
  const addCleanup = useContext(CleanupContext);

  const [, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    const interval = setInterval(() => reload(r => !r), 100); //just swapping between true/false
    addCleanup(() => clearInterval(interval));
    return () => clearInterval(interval);
  }, []);

  const server = ns.self().server;
  const explorerScript = config.get('explorer');
  const desktop = server + "/" + (config.get("desktop") ?? '');

  return <div className='plasma-desktop'>
    <DoubleClickFileContext.Provider value={(_, { type, name }) => {
      switch (type) {
        case 'js':
          ns.run(name);
          break;
        case 'folder':
          explorerScript && ns.run(explorerScript, undefined, `${server}/${name}`);
          break;
        case 'txt':
          ns.alert(readFile(ns, `${server}/${name}`));
          break;
        case 'exe':
          ns.toast('.exe files can only be run from the terminal', 'error');
          break;
        default:
          ns.toast(`This filetype is not supported (${type})`, 'error');
          break;
      }
    }}>

      <FileGrid path={server} files={read_dir(ns, desktop)} ></FileGrid>
    </DoubleClickFileContext.Provider>
  </div>;
}