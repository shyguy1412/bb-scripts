import { PathContext } from '..';
import Style from './BreadCrumbs.css';
import React, { useContext } from "react";
import { List } from '@/lib/components/List';
import { DropTarget } from '@/lib/components/DropTarget';
import { transferFile } from '@/lib/FileSystem';
import { NetscriptContext } from '@/lib/Context';

export function BreadCrumbs() {

  const [path] = useContext(PathContext);

  return <div className="dolphin-bread-crumbs"><Style></Style>
    <List data={path.split('/').map((crumb, i, arr) => ({ crumb, path: arr.slice(0, i + 1).join('/') }))} li={Crumb}></List>
  </div>;
}

function Crumb({ crumb, path }: { crumb: string; path: string; }) {
  const [, setPath] = useContext(PathContext);
  const ns = useContext(NetscriptContext);

  return <span style={{userSelect: 'none'}}>
    <DropTarget
      className='dolphin-bread-crumb-clickable'
      accept='file'
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
      onClick={() => {
        setPath(path);
      }}
    >{crumb}</DropTarget>
    /
  </span>;
}