import style from './FileTile.css' with {'type': 'css'};
import React, { MouseEvent, createContext, useContext } from 'react';
import { DragTarget } from '@/lib/components/DragTarget';
import { NetscriptContext } from '@/lib/Context';

import { FaFileCode, FaPen } from 'react-icons/fa';
import { FaFileLines, FaFolderClosed, FaTrashCan } from 'react-icons/fa6';
import { adoptStyle } from '@/lib/BitburnerDOM';

// export const DoubleClickFileContext = createContext<(() => { });

const FileIcon = {
  'js': () => <FaFileCode></FaFileCode>,
  'exe': () => <FaFileCode></FaFileCode>,
  'txt': () => <FaFileLines></FaFileLines>,
  'msg': () => <FaFileLines></FaFileLines>,
  'folder': () => <FaFolderClosed></FaFolderClosed>,
} as const;
type FileIcon = keyof typeof FileIcon;

export namespace FileTile {
  export type File = {
    path: string; name: string; type: FileIcon;
  };

  export type Props = {
    file: {
      name: string,
      type: 'file' | 'folder';
    };
    path: string;
    onDoubleClick?: (e: MouseEvent<HTMLDivElement>, file: File) => void;
  };
}

export function FileTile({ file, path, onDoubleClick }: FileTile.Props) {
  const ns = useContext(NetscriptContext);
  const type = (file.type == 'file' ? file.name.split('.').at(-1) : 'folder') as FileIcon;
  const Icon = FileIcon[type] ?? FileIcon['txt'];

  adoptStyle(ns, style);

  const preventDragDefault: React.DragEventHandler<HTMLDivElement> = (e) => {
    if (e.dataTransfer.types.includes('file') && type == 'folder') e.preventDefault();
  };

  const onDrop: React.DragEventHandler<HTMLDivElement> = (e) => {
    if (type != 'folder') return;
    const [sourceServer, sourceFile] = e.dataTransfer.getData('data').split(/\/(.*)/, 2);
    const [targetServer] = path.split('/');
    // transferFile(
    //   ns,
    //   `${sourceServer}/${sourceFile}`,
    //   `${path}/${file.name}/${sourceFile.split('/').at(-1)}`,
    //   sourceServer != targetServer
    // );
  };

  return <DragTarget
    className='file-tile'
    group={`${type != 'folder' ? 'file' : 'folder'}`}
    data={`${path}/${file.name}`}
    onDoubleClick={(e) => onDoubleClick?.(e, { name: file.name, path, type })}
    onDragEnter={preventDragDefault}
    onDragOver={preventDragDefault}
    onDrop={onDrop}
  >
    <Icon></Icon>

    <div
      className='file-name'
      spellCheck={false}
      onDoubleClick={() => void 0}
    >{file.name}</div>

    <div className='file-action-buttons'>

    </div>
  </DragTarget>;
};


// (e) => {
//         e.stopPropagation();
//         const el = e.currentTarget as HTMLDivElement;
//         el.contentEditable = 'true';
//         el.focus();
//         const keydownHandler = (e: KeyboardEvent) => {
//           if (e.key == 'Enter') {
//             el.contentEditable = 'false';
//             window.focus();
//           }
//         };
//         el.addEventListener('keydown', keydownHandler);
//         el.addEventListener('focusout', ({ currentTarget }) => {
//           const newName = (currentTarget as HTMLDivElement).textContent;
//           if (newName == file.name) return;
//           try {
//             // if (type == 'folder')
//             // moveFolder(ns, `${path}/${file.name}`, `${path}/${newName}`);
//             // else
//             // moveFile(ns, `${path}/${file.name}`, `${path}/${newName}`);
//           } catch (e) {
//             console.log({ e });
//             ns.toast((e as Error).name, 'error');
//           }
//           el.removeEventListener('keydown', keydownHandler);
//         }, { once: true });
//       }



// {['js', 'txt'].includes(type) ?
//   <FaPen
//     style={{ cursor: 'pointer', fontSize: '0.9em' }}
//     onClick={async () => {
//       // const term = new Terminal(ns);
//       // const [server, filepath] = `${path}/${file.name}`.split(/\/(.*)/, 2);
//       // if (!term.terminalInput) return ns.toast('Editing can only be triggered when the terminal tab is selected', 'warning');
//       // term.exec(getConnectionPath(ns, server).reduce((prev, cur) => prev + `connect ${cur};`, ''));
//       // term.exec(`nano ${filepath}`);
//       // term.cleanup();
//     }}></FaPen> : <span></span>}
// {['js', 'txt', 'folder'].includes(type) ?
//   <FaTrashCan
//     style={{ cursor: 'pointer', fontSize: '0.9em' }}
//     onClick={async () => {
//       if (! await ns.prompt(`Are you sure you want to delete ${path}/${file.name}?`, { type: 'boolean' })) return;
//       // if (type == 'folder')
//       // delete_folder(ns, `${path}/${file.name}`);
//       // else
//       // deleteFile(ns, `${path}/${file.name}`);
//     }}></FaTrashCan> : undefined}