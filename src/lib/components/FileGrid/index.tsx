import Style from './FileGrid.css';
import { useContext } from 'react';
import React from 'react';
import { readDir, transferFile } from '@/lib/FileSystem';
import { List } from '@/lib/components/List';
import { DropTarget } from '@/lib/components/DropTarget';
import { FileTile } from '@/lib/components/FileTile';
import { NetscriptContext } from '@/lib/Context';

type Props = {
  files: ReturnType<typeof readDir>;
  path: string;
};

export function FileGrid({ path, files }: Props) {
  const ns = useContext(NetscriptContext);

  return <>
    <Style></Style>
    <DropTarget
      accept='file'
      className='file-grid-drop-target'
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
      <div className='file-grid'>
        <List data={files.filter(f => !f.name.startsWith('.')).map(file => ({ file, path }))} li={FileTile}></List>
      </div>
    </DropTarget>
  </>;
}