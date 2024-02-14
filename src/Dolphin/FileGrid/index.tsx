import Style from './FileGrid.css';
import { useContext } from 'react';
import { PathContext, ReloadContext } from '@/Dolphin';
import React from 'react';
import { readDir, transferFile } from '@/lib/FileSystem';
import { List } from '@/lib/components/List';
import { FileTile } from '@/Dolphin/FileTile';
import { DropTarget } from '@/lib/components/DropTarget';
import { NetscriptContext } from '@/lib/Context';

export function FileGrid() {
  const ns = useContext(NetscriptContext);
  const [path] = useContext(PathContext);
  const reload = useContext(ReloadContext);
  const files = readDir(ns, path);

  return <>
    <Style></Style>
    <DropTarget
      accept='dolphin-file'
      className='dolphin-file-grid-drop-target'
      onDrop={(e) => {
        const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
        const [targetServer] = path.split('/');
        transferFile(
          ns,
          `${sourceServer}/${sourceFile}`,
          `${path}/${sourceFile.split('/').at(-1)}`,
          sourceServer != targetServer
        ); reload();
      }}
    >
      <div className='dolphin-file-grid'>
        <List data={files.map(file => ({ file }))} li={FileTile}></List>
      </div>
    </DropTarget>
  </>;
}