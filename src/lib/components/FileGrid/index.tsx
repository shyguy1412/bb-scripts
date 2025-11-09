import style from './FileGrid.css' with {'type': 'css'};
import { useContext } from 'react';
import React from 'react';
import { DirEnt } from '@/lib/FileSystem';
import { List } from '@/lib/components/List';
import { DropTarget } from '@/lib/components/DropTarget';
import { FileTile } from '@/lib/components/FileTile';
import { NetscriptContext } from '@/lib/Context';
import { useStyle } from '@/lib/hooks/useStyle';

type Props = {
  files: DirEnt[];
  path: string;
};

export function FileGrid({ path, files }: Props) {
  const ns = useContext(NetscriptContext);

  useStyle(style);

  if (!files || !path) return undefined;
  return <>
    <DropTarget
      accept={['file', 'folder']}
      className='file-grid-drop-target'
      onDrop={(e) => {
        // const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
        // const [targetServer] = path.split('/');
        // if (e.dataTransfer.types.includes('file'))
        //   // transferFile(
        //   //   ns,
        //   //   `${sourceServer}/${sourceFile}`,
        //   //   `${path}/${sourceFile.split('/').at(-1)}`,
        //   //   sourceServer != targetServer
        //   // );
        // // else {
        //   // transferFolder(
        //   //   ns,
        //   //   `${sourceServer}/${sourceFile}`,
        //   //   `${path}/${sourceFile.split('/').at(-1)}`,
        //   //   sourceServer != targetServer
        //   // );
        // }
      }}
    >
      <div className='file-grid'>
        <List data={files.filter(f => !f.name.startsWith('.')).map(file => ({ file, path }))} li={FileTile}></List>
      </div>
    </DropTarget>
  </>;
}