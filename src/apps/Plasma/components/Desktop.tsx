import { ConfigContext } from '../main';
import { NetscriptContext } from '@/lib/Context';
import { read_dir } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import { connect_to_fdaemon } from '@/servers/home/bin/service/fdaemon';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {
  'use run';

  const ns = useContext(NetscriptContext);
  const config = useContext(ConfigContext);
  const server = ns.self().server;
  const explorerScript = config.get('explorer');
  const desktop = config.get("desktop") ?? '';

  const [files, setFiles] = useState(read_dir(ns, desktop));

  useEffect(() => {
    const [write, read] = connect_to_fdaemon(ns);
    write("subscribe", {
      event: 'change',
      path: desktop
    });
    read().finally(() => setFiles(read_dir(ns, desktop)));
  }, [files]);


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
          ns.alert(ns.read(name));
          break;
        case 'exe':
          ns.toast('.exe files can only be run from the terminal', 'error');
          break;
        default:
          ns.toast(`This filetype is not supported (${type})`, 'error');
          break;
      }
    }}>

      <FileGrid path={server} files={files} ></FileGrid>
    </DoubleClickFileContext.Provider>
  </div>;
}