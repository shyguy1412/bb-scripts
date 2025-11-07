import { getAllServers } from '@/lib/Network';

type DirEnt = { name: string, type: 'file' | 'folder'; };

export const ALLOWED_FILETYPES = [
  'js', 'txt', 'json'
];

export function getAllCodingContracts(ns: NS) {
  'use ls';
  const files: { name: string, type: 'file'; }[] = [];
  for (const server of getAllServers(ns)) {
    for (const file of ns.ls(server)) {
      if (file.endsWith('.cct')) {
        files.push({ name: file, type: 'file' });
      }
    }
  }
  console.log(`${files.length} coding contracts found`);

  return files;
}

export function read_dir(ns: NS, path: string, server = ns.self().server): DirEnt[] {
  const files: DirEnt[] = ns.ls(server, path)
    .map(f => ('/' + f).replace(new RegExp(`/?${path}/?([^/]*).*`), '$1'))
    .map(name => ({
      name,
      type: name.includes('.') ? 'file' : 'folder'
    }));

  return files;
};

type DirEntry = NonNullable<ReturnType<typeof read_dir>>[number] & { path: string; };
export function readDirRecursive(ns: NS, path: string): DirEntry[] {
  const content = read_dir(ns, path);
  if (!content) throw new Error('Path does not exists: ' + path);
  return content
    .flatMap(file => file.type == 'folder' ? readDirRecursive(ns, `${path}/${file.name}`).flat() : file as DirEntry)
    .map(f => ({ ...f, path: f.path ?? path }));
};

export function mkdir(ns: NS, path: string) {
  writeFile(ns, '', `${path}/.keepdir.txt`);
}

export function readFile(ns: NS, path: string) {
  'use getHostname';
  'use scp';
  'use rm';
  const [server, file] = path.split(/\/(.*)/, 2);

  if (server == ns.getHostname()) {
    return ns.read(file);
  }

  ns.scp(file, ns.getHostname(), server);
  const content = ns.read(file);
  ns.rm(file);
  return content;
}

export function isFile(path: string) {
  return /\..?.?.?$/.test(path);
}

export function writeFile(ns: NS, content: string, path: string) {
  'use scp';
  const extension = path.split('.').at(-1);

  if (extension != 'js' && extension != 'txt') {
    throw new Error('only scripts and textfiles can be moved');
  }

  const [server, file] = path.split(/\/(.*)/, 2);

  if (!file) {
    throw new Error('path is not a file');
  }

  const tempname = `fs_temp_${Date.now()}.txt`;
  ns.write(tempname, content);
  moveFile(ns, `${ns.getHostname()}/${tempname}`, path);
};

export function moveFile(ns: NS, source: string, destination: string) {
  const sourceExtension = source.split('.').at(-1);
  if (sourceExtension != 'js' && sourceExtension != 'txt') {
    throw new Error('only scripts and textfiles can be moved');
  }
  const [sourceServer, sourceFile] = source.split(/\/(.*)/, 2);
  const [destinationServer, destinationFile] = destination.split(/\/(.*)/, 2);

  if (sourceServer == destinationServer) ns.mv(sourceServer, sourceFile, destinationFile);
  else {
    copyFile(ns, source, destination);
    deleteFile(ns, source);
  }
}


export function moveFolder(ns: NS, source: string, destination: string) {
  const folder = readDirRecursive(ns, source);

  for (const file of folder) {
    moveFile(ns, `${file.path}/${file.name}`, `${file.path.replace(source, destination)}/${file.name}`);
  }
}

export function copyFile(ns: NS, source: string, destination: string) {
  'use scp';
  const sourceExtension = source.split('.').at(-1);

  if (sourceExtension != 'js' && sourceExtension != 'txt') {
    throw new Error('only scripts and textfiles can be moved');
  }

  const [sourceServer, sourceFile] = source.split(/\/(.*)/, 2);
  const [destinationServer, destinationFile] = destination.split(/\/(.*)/, 2);

  if (!sourceFile) {
    throw new Error('source is not a file');
  }

  if (!destinationFile) {
    throw new Error('destination is not a file');
  }

  if (sourceServer == destinationServer) ns.write(destinationFile, ns.read(sourceFile), 'w');
  else {
    ns.scp(sourceFile, destinationServer, sourceServer);
    ns.mv(destinationServer, sourceFile, destinationFile);
  };
}

export function deleteFile(ns: NS, file: string) {
  'use rm';
  const [server, path] = file.split(/\/(.*)/, 2);
  ns.rm(path, server);
}

export function deleteFolder(ns: NS, path: string) {
  const folder = readDirRecursive(ns, path);

  for (const file of folder) {
    deleteFile(ns, `${file.path}/${file.name}`);
  }
}

export function transferFile(ns: NS, source: string, destination: string, copy?: boolean) {
  if (copy) copyFile(ns, source, destination);
  else moveFile(ns, source, destination);
}

export function transferFolder(ns: NS, source: string, destination: string, copy?: boolean) {
  const folder = readDirRecursive(ns, source);

  for (const file of folder) {
    transferFile(ns, `${file.path}/${file.name}`, `${file.path.replace(source, destination)}/${file.name}`, copy);
  }
}
