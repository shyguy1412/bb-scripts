import { FileGrid } from '@/Dolphin/FileGrid';
import Style from './Dolphin.css';
import { ServerSection } from '@/Dolphin/ServerSection';
import { List } from '@/lib/components/List';
import { getAllServers } from '@/lib/Network';
import React, { createContext, useReducer, useState } from 'react';
import { BreadCrumbs } from '@/Dolphin/BreadCrumbs';
import { moveFile, copyFile } from '@/lib/FileSystem';

type Props = {
  ns: NS;
};

export const PathContext = createContext<ReturnType<typeof useState<string>>>(null);
export const ReloadContext = createContext<() => void>(null);

export function Dolphin({ ns }: Props) {

  'use getHostname';
  const [path, setPath] = useState<string>(ns.args[0] as string ?? ns.getHostname());
  const [, reload] = useReducer(() => ({}), {});

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

  return <>
    <Style></Style>
    <PathContext.Provider value={[path, setPath]}>
      <ReloadContext.Provider value={reload}>

        <div className='dolphin-layout'>
          <div className='dolphin-explorer'>
            <List data={sections.map(s => ({ ...s }))} li={ServerSection} ></List>
          </div>
          <div className='dolphin-content'>
            <BreadCrumbs></BreadCrumbs>
            <FileGrid></FileGrid>
          </div>
        </div>
      </ReloadContext.Provider>
    </PathContext.Provider>
  </>;
}