export function readDir(ns: NS, path: string) {
  'use ls';
  try {

    const [server, ...directory] = path.split('/');

    const filesystemTree = ns.ls(server)
      .reduce((prev, cur) => {
        const path = cur.split('/');
        let node = prev;
        while (path.length) {
          const curNode = path.shift();
          node[curNode] ??= {};
          node = node[curNode];
        };
        return prev;
      }, {});

    const folder = directory.reduce((prev, cur) => prev[cur], filesystemTree);
    const filesWithType = Object.keys(folder)
      .map(key => ({
        name: key,
        type: Object.keys(folder[key]).length ? 'folder' : 'file' as 'file' | 'folder'
      }));

    return filesWithType;
  } catch {
    return null;
  }
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
  const readDirRecursive = (path: string): (ReturnType<typeof readDir>[number] & { path: string; })[] => {
    const content = readDir(ns, path);
    return content
      .map(file => file.type == 'folder' ? readDirRecursive(`${path}/${file.name}`).flat() : file as (ReturnType<typeof readDir>[number] & { path?: string; }))
      .flat()
      .map(f => ({ ...f, path: f.path ?? path }));
  };
  const folder = readDirRecursive(source);

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

export function transferFile(ns: NS, source: string, destination: string, copy?: boolean) {
  if (copy) copyFile(ns, source, destination);
  else moveFile(ns, source, destination);
}
