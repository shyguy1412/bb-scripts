import Style from './FileTile.css';
import { faFileCode, faFileLines, faFolderClosed } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { MouseEvent, createContext, useContext } from 'react';
import { moveFile, transferFile } from '@/lib/FileSystem';
import { DragTarget } from '@/lib/components/DragTarget';
import { NetscriptContext } from '@/lib/Context';

type Props = {
  file: {
    name: string,
    type: 'file' | 'folder';
  };
  path: string;
};

export const DoubleClickFileContext = createContext<(e: MouseEvent<HTMLDivElement>, file: { path: string; name: string; type: keyof typeof FileIcons; }) => void>(() => { });

const FileIcons = {
  'js': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'exe': () => <FontAwesomeIcon icon={faFileCode}></FontAwesomeIcon>,
  'txt': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'msg': () => <FontAwesomeIcon icon={faFileLines}></FontAwesomeIcon>,
  'folder': () => <FontAwesomeIcon icon={faFolderClosed}></FontAwesomeIcon>
} as const;

export function FileTile({ file, path }: Props) {
  'use exec';

  const ns = useContext(NetscriptContext);
  const type = (file.type == 'file' ? file.name.split('.').at(-1) : 'folder') as keyof typeof FileIcons;
  const Icon = FileIcons[type] ?? FileIcons['txt'];
  const onDoubleClick = useContext(DoubleClickFileContext);

  return <DragTarget
    className='file-tile'
    group={`${type != 'folder' ? 'file' : 'folder'}`}
    data={`${path}/${file.name}`}
    onDoubleClick={(e) => onDoubleClick(e, { name: file.name, path, type })}
    onDragEnter={(e) => {
      if (e.dataTransfer.types.includes('file') && type == 'folder') e.preventDefault();
    }}
    onDragOver={(e) => {
      if (e.dataTransfer.types.includes('file') && type == 'folder') e.preventDefault();
    }}
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
    <div
      spellCheck={false}
      onDoubleClick={(e) => {
        if (type == 'folder') return;
        e.stopPropagation();

        const el = e.currentTarget as HTMLDivElement;
        el.contentEditable = 'true';
        el.focus();

        const keydownHandler = (e: KeyboardEvent) => {
          if (e.key == 'Enter') {
            el.contentEditable = 'false';
            window.focus();
          }
        };

        el.addEventListener('keydown', keydownHandler);

        el.addEventListener('focusout', ({ currentTarget }) => {
          const newName = (currentTarget as HTMLDivElement).textContent;
          if (newName == file.name) return;

          try {
            moveFile(ns, `${path}/${file.name}`, `${path}/${newName}`);
          } catch (e) {
            console.log({ e });

            ns.toast((e as Error).name, 'error');
          }

          el.removeEventListener('keydown', keydownHandler);

        }, { once: true });

      }}>{file.name}</div>
  </DragTarget>;
};
