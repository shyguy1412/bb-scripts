import { PathContext, ReloadContext } from '@/Dolphin';
import Style from './FileTile.css';
import { faFileCode, faFileLines, faFolderClosed } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';
import { NetscriptContext } from '@/lib/WindowApp';
import { copyFile, moveFile, readFile, transferFile } from '@/lib/FileSystem';
import { DragTarget } from '@/lib/components/DragTarget';
import { DropTarget } from '@/lib/components/DropTarget';

type Props = {
  file: string;
};

const FileIcons = {
  'js': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'exe': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'txt': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'msg': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'folder': () => <FontAwesomeIcon icon={faFolderClosed}></FontAwesomeIcon>
} as const;

export function FileTile({ file }: Props) {

  const ns = useContext(NetscriptContext);
  const [path, setPath] = useContext(PathContext);
  const reload = useContext(ReloadContext);
  const type = (file.includes('.') ? file.split('.').at(-1) : 'folder') as keyof typeof FileIcons;
  const Icon = FileIcons[type] ?? FileIcons['txt'];

  return <DragTarget
    className='dolphin-file-tile'
    group={`dolphin-${type != 'folder' ? 'file' : 'folder'}`}
    data={`${path}/${file}`}
    onDoubleClick={() => {
      switch (type) {
        case 'js':
          'use exec';
          const [server, script] = `${path}/${file}`.split(/\/(.*)/, 2);
          ns.exec(script, server);
          break;
        case 'folder':
          setPath(`${path}/${file}`);
          break;
        case 'txt':
          ns.alert(readFile(ns, `${path}/${file}`));
          break;
        case 'exe':
          ns.toast('.exe files can not be run from Dolphin', 'error');
          break;
        default:
          // ns.
          ns.toast('Dolphin does not support this filetype', 'error');
          break;
      }
    }}
    onDragEnter={(e) => {
      if (e.dataTransfer.types.includes('dolphin-file') && type == 'folder') e.preventDefault();
    }}
    onDragOver={(e) => {
      if (e.dataTransfer.types.includes('dolphin-file') && type == 'folder') e.preventDefault();
    }}
    onDragEnd={() => reload()}
    onDrop={(e) => {
      if (type != 'folder') return;
      const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
      const [targetServer] = path.split('/');
      transferFile(
        ns,
        `${sourceServer}/${sourceFile}`,
        `${path}/${file}/${sourceFile.split('/').at(-1)}`,
        sourceServer != targetServer
      ); reload();
    }}
  >
    <Style></Style>
    <Icon></Icon>
    <div>{file}</div>
  </DragTarget>;
};
