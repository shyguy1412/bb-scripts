import { ConfigContext } from '../main';
import { NetscriptContext } from '@/lib/Context';
import { list_directory } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import { connect_to_fdaemon } from '@/servers/home/bin/service/fdaemon';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {
  const ns = useContext(NetscriptContext);
  const config = useContext(ConfigContext);
  const server = ns.self().server;
  const explorerScript = config.get('explorer');
  const desktop = config.get("desktop") ?? '';

  const [files, setFiles] = useState(list_directory(ns, desktop, ns.self().server, { withFileTypes: true }));

  useEffect(() => {
    const [write, read] = connect_to_fdaemon(ns);
    write("subscribe", {
      event: 'change',
      path: desktop
    });
    read().finally(() => setFiles(list_directory(ns, desktop, ns.self().server, { withFileTypes: true })));
  }, [files]);

  const onDoubleClick: FileGrid.Props["onDoubleClick"] = (_, { type, name }) => {
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
  };

  return <div className='plasma-desktop'>
    <FileGrid path={server} files={files} onDoubleClick={onDoubleClick} ></FileGrid>
  </div>;
}