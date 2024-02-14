import Style from './FileTile.css';
import { faFileCode, faFileLines, faFolderClosed } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';
import { readFile, transferFile } from '@/lib/FileSystem';
import { DragTarget } from '@/lib/components/DragTarget';
import { NetscriptContext } from '@/lib/Context';

type Props = {
  file: {
    name: string,
    type: 'file' | 'folder';
  };
  path: string;
};
const FileIcons = {
  'js': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'exe': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'txt': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'msg': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'folder': () => <FontAwesomeIcon icon={faFolderClosed}></FontAwesomeIcon>
} as const;

export function FileTile({ file, path }: Props) {

  const ns = useContext(NetscriptContext);
  const type = (file.type == 'file' ? file.name.split('.').at(-1) : 'folder') as keyof typeof FileIcons;
  const Icon = FileIcons[type] ?? FileIcons['txt'];

  return <DragTarget
    className='file-tile'
    group={`${type != 'folder' ? 'file' : 'folder'}`}
    data={`${path}/${file.name}`}
    onDoubleClick={() => {
      switch (type) {
        case 'js':
          'use exec';
          const [server, script] = `${path}/${file.name}`.split(/\/(.*)/, 2);
          ns.exec(script, server);
          break;
        case 'folder':
          ns.exec('Dolphin.js', path, 1, `${path}/${file.name}`);
          break;
        case 'txt':
          ns.alert(readFile(ns, `${path}/${file.name}`));
          break;
        case 'exe':
          ns.toast('.exe files can only be run from the terminal', 'error');
          break;
        default:
          ns.toast(`This filetype is not supported (${type})`, 'error');
          break;
      }
    }}
    onDragEnter={(e) => {
      if (e.dataTransfer.types.includes('file') && type == 'folder') e.preventDefault();
    }}
    onDragOver={(e) => {
      if (e.dataTransfer.types.includes('file') && type == 'folder') e.preventDefault();
    }}
    // onDragEnd={() => reload()}
    onDrop={(e) => {
      if (type != 'folder') return;
      const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
      const [targetServer] = path.split('/');
      transferFile(
        ns,
        `${sourceServer}/${sourceFile}`,
        `${path}/${file.name}/${sourceFile.split('/').at(-1)}`,
        sourceServer != targetServer
      );
    }}
  >
    <Style></Style>
    <Icon></Icon>
    <div>{file.name}</div>
  </DragTarget>;
};
