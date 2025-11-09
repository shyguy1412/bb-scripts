import { PathContext } from '..';
import { DropTarget } from '@/lib/components/DropTarget';
import { NetscriptContext } from '@/lib/Context';
// import { transferFile } from '@/lib/FileSystem';
import { Server } from 'NetscriptDefinitions';
import React, { useContext } from 'react';

type Props = {
  server: Server;
};

export function ServerEntry({ server }: Props) {
  const ns = useContext(NetscriptContext);
  const [, setPath] = useContext(PathContext) ?? [];

  return <DropTarget
    accept='file'
    className='dolphin-explorer-button'
    onClick={() => setPath(server.hostname)}
    onDrop={(e) => {
      const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
      // transferFile(
      //   ns,
      //   `${sourceServer}/${sourceFile}`,
      //   `${server.hostname}/${sourceFile.split('/').at(-1)}`,
      //   sourceServer != server.hostname
      // );
    }}
  >{server.hostname}</DropTarget>;
}
