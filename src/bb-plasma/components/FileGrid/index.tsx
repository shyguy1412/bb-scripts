import Style from './FileGrid.css';
import { useContext } from 'react';
import React from 'react';
import { readDir, transferFile } from '@/lib/FileSystem';
import { List } from '@/lib/components/List';
import { DropTarget } from '@/lib/components/DropTarget';
import { NetscriptContext } from '@/bb-plasma/DesktopEnviroment';
import { FileTile } from '@/bb-plasma/components/FileTile';

export function FileGrid() {
  const ns = useContext(NetscriptContext);
  const path = ns.getHostname();
  const files = readDir(ns, path);

  return <>
    <Style></Style>
    <DropTarget
      accept='dolphin-file'
      className='plasma-file-grid-drop-target'
      onDrop={(e) => {
        const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
        const [targetServer] = path.split('/');
        transferFile(
          ns,
          `${sourceServer}/${sourceFile}`,
          `${path}/${sourceFile.split('/').at(-1)}`,
          sourceServer != targetServer
        );
      }}
    >
      <div className='plasma-file-grid'>
        <List data={files.map(file => ({ file }))} li={FileTile}></List>
      </div>
    </DropTarget>
  </>;
}