import { PathContext, ReloadContext } from '@/Dolphin';
import { NetscriptContext } from '@/lib/WindowApp';
import { DropTarget } from '@/lib/components/DropTarget';
import { copyFile, moveFile, transferFile } from '@/lib/FileSystem';
import { Server } from 'NetscriptDefinitions';
import React, { useContext } from 'react';

type Props = {
  server: Server;
};

export function ServerEntry({ server }: Props) {
  const ns = useContext(NetscriptContext);
  const [, setPath] = useContext(PathContext) ?? [];
  const reload = useContext(ReloadContext);

  return <DropTarget
    accept='dolphin-file'
    className='dolphin-explorer-button'
    onClick={() => setPath(server.hostname)}
    onDrop={(e) => {
      const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
      transferFile(
        ns,
        `${sourceServer}/${sourceFile}`,
        `${server.hostname}/${sourceFile.split('/').at(-1)}`,
        sourceServer != server.hostname
      ); reload();
    }}
  >{server.hostname}</DropTarget>;
}
