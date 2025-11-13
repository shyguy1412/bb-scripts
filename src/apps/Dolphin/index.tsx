import { FileGrid } from '@/lib/components/FileGrid';
import style from './Dolphin.css' with {type: "css"};
import { ServerSection } from './ServerSection';
import { List } from '@/lib/components/List';
import { getAllServers } from '@/lib/Network';
import React, { Dispatch, SetStateAction, createContext, useContext, useEffect, useMemo, useState } from 'react';
import { BreadCrumbs } from './BreadCrumbs';
import { read_dir } from '@/lib/FileSystem';
import { DoubleClickFileContext } from '@/lib/components/FileTile';
import { NetscriptContext } from '@/lib/Context';
import { getAllCodingContracts } from '@/lib/FileSystem';
import { useReload } from '@/lib/hooks/useReload';
import { FaPlus } from 'react-icons/fa';
import { adoptStyle } from '@/lib/hooks/useStyle';
import { enable_hot_reload } from '@/lib/syscalls/hot_reload';

// @ts-expect-error
export const PathContext = createContext<([string, Dispatch<SetStateAction<string>>])>(null);

export function Dolphin() {
  'use exec';
  'use getHostname';
  'use getServer';

  const ns = useContext(NetscriptContext);

  useReload();
  enable_hot_reload(ns);
  adoptStyle(style);

  const [path, setPath] = useState<string>(ns.args[0] as string ?? ns.getHostname());

  const servers = getAllServers(ns);
  const sections = servers.reduce((prev, cur) => {
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

  ns.ui.setTailTitle(`Dolphin - ${path.replace(/([^\/]*)(\/?)/, '$1://')}`);

  const [_, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    if (path == '~home/coding-contracts') return;
    const timeout = setTimeout(() => reload(!_), 500); //just swapping between true/false
    return () => clearTimeout(timeout);
  });

  const files = useMemo(() => path == '~home/coding-contracts' ? getAllCodingContracts(ns) : read_dir(ns, path), [path]);

  if (files)
    return <>
      <PathContext.Provider value={[path, setPath]}>
        <div className='dolphin-layout'>
          <div className='dolphin-actionbar'>
            <BreadCrumbs></BreadCrumbs>
            <span className='dolphin-actions'>
              <span
                onClick={() => {
                  // mkdir(ns, `${path}/new_dir`);
                }}
              >
                <FaPlus style={{ marginRight: '0.2em' }}></FaPlus>
                new folder
              </span>
              <span
                onClick={() => {
                  // writeFile(ns, '', `${path}/new_file.js`);
                }}
              >
                <FaPlus style={{ marginRight: '0.2em' }}></FaPlus>
                new file
              </span>
            </span>
          </div>
          <div className='dolphin-explorer'>
            <List data={sections.map(s => ({ ...s }))} li={ServerSection} ></List>
            <div
              className='dolphin-explorer-button'
              onClick={() => (setPath('~home/coding-contracts'))}
            >Coding Contracs</div>
          </div>
          <div className='dolphin-content'>
            <DoubleClickFileContext.Provider value={(e, { type, name }) => {
              switch (type) {
                case 'js':
                  const [server, script] = `${path}/${name}`.split(/\/(.*)/, 2);
                  ns.exec(script, server);
                  break;
                case 'folder':
                  setPath(`${path}/${name}`);
                  break;
                case 'txt':
                  // ns.alert(readFile(ns, `${path}/${name}`));
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