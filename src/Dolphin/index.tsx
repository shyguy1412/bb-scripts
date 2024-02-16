import { FileGrid } from '@/lib/components/FileGrid';
import Style from './Dolphin.css';
import { ServerSection } from '@/Dolphin/ServerSection';
import { List } from '@/lib/components/List';
import { getAllServers } from '@/lib/Network';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { BreadCrumbs } from '@/Dolphin/BreadCrumbs';
import { readDir, readFile } from '@/lib/FileSystem';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import { NetscriptContext } from '@/lib/Context';


export const PathContext = createContext<ReturnType<typeof useState<string>>>(null);

export function Dolphin() {

  const ns = useContext(NetscriptContext);

  'use getHostname';
  const [path, setPath] = useState<string>(ns.args[0] as string ?? ns.getHostname());

  const servers = getAllServers(ns);
  const sections = servers.reduce((prev, cur) => {
    'use getServer';
    const server = ns.getServer(cur);

    if (server.hostname == 'home') {
      prev.unshift({ servers: [server] });
      return prev;
    }

    if (server.purchasedByPlayer) {
      const label = 'purchased';
      const section = prev.find(s => s.section == label);
      if (section) section.servers.push(server);
      else prev.push({
        section: label,
        servers: [server],
      });
      return prev;
    }

    if (!server.purchasedByPlayer) {
      const label = 'other';
      const section = prev.find(s => s.section == label);
      if (section) section.servers.push(server);
      else prev.push({
        section: label,
        servers: [server],
      });
      return prev;
    }

    return prev;
  }, [] as Parameters<typeof ServerSection>[0][]);

  ns.setTitle(`Dolphin - ${path.replace(/([^\/]*)(\/?)/, '$1://')}`);

  const [_, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    const timeout = setTimeout(() => reload(!_), 500); //just swapping between true/false
    return () => clearTimeout(timeout);
  });

  const files = readDir(ns, path);
  if (files)
    return <>
      <Style></Style>
      <PathContext.Provider value={[path, setPath]}>
        <div className='dolphin-layout'>
          <div className='dolphin-explorer'>
            <List data={sections.map(s => ({ ...s }))} li={ServerSection} ></List>
          </div>
          <div className='dolphin-content'>
            <BreadCrumbs></BreadCrumbs>
            <DoubleClickFileContext.Provider value={(e, { type, name }) => {
              switch (type) {
                case 'js':
                  'use exec';
                  const [server, script] = `${path}/${name}`.split(/\/(.*)/, 2);
                  ns.exec(script, server);
                  break;
                case 'folder':
                  setPath(`${path}/${name}`);
                  break;
                case 'txt':
                  ns.alert(readFile(ns, `${path}/${name}`));
                  break;
                case 'exe':
                  ns.toast('.exe files can only be run from the terminal', 'error');
                  break;
                default:
                  ns.toast(`This filetype is not supported (${type})`, 'error');
                  break;
              }
            }}
            >
              <FileGrid files={files} path={path}></FileGrid>
            </DoubleClickFileContext.Provider>
          </div>
        </div>
      </PathContext.Provider>
    </>;

  ns.toast("Current folder was deleted, moving to home", "warning");

  return setPath('home') as undefined;
}