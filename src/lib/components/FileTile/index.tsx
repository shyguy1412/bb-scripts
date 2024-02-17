import Style from './FileTile.css';
import { faFileCode, faFileLines, faFolderClosed, faPen, faTrash, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { MouseEvent, createContext, useContext } from 'react';
import { deleteFile, deleteFolder, moveFile, moveFolder, transferFile } from '@/lib/FileSystem';
import { DragTarget } from '@/lib/components/DragTarget';
import { NetscriptContext } from '@/lib/Context';
import { Terminal } from '@/lib/Terminal';
import { NS } from 'NetscriptDefinitions';
import { getConnectionPath } from '@/lib/Network';

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
      className='file-name'
      spellCheck={false}
      onDoubleClick={(e) => {
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
            if (type == 'folder')
              moveFolder(ns, `${path}/${file.name}`, `${path}/${newName}`);
            else
              moveFile(ns, `${path}/${file.name}`, `${path}/${newName}`);
          } catch (e) {
            console.log({ e });
            ns.toast((e as Error).name, 'error');
          }
          el.removeEventListener('keydown', keydownHandler);
        }, { once: true });
      }}>{file.name}</div>
    <div className='file-action-buttons'>
      {['js', 'txt'].includes(type) ?
        <FontAwesomeIcon
          icon={faPen}
          style={{ cursor: 'pointer', fontSize: '0.9em' }}
          onClick={async () => {
            const term = new Terminal(ns);
            const [server, filepath] = `${path}/${file.name}`.split(/\/(.*)/, 2);
            if (!term.terminalInput) return ns.toast('Editing can only be triggered when the terminal tab is selected', 'warning');
            term.exec(getConnectionPath(ns, server).reduce((prev, cur) => prev + `connect ${cur};`, ''));
            term.exec(`nano ${filepath}`);
            term.cleanup();
          }}></FontAwesomeIcon> : <span></span>}
      {['js', 'txt', 'folder'].includes(type) ?
        <FontAwesomeIcon
          icon={faTrashCan}
          style={{ cursor: 'pointer', fontSize: '0.9em' }}
          onClick={async () => {
            if (! await ns.prompt(`Are you sure you want to delete ${path}/${file.name}?`, { type: 'boolean' })) return;
            if (type == 'folder')
              deleteFolder(ns, `${path}/${file.name}`);
            else
              deleteFile(ns, `${path}/${file.name}`);
          }}></FontAwesomeIcon> : undefined}
    </div>
  </DragTarget>;
};
