export function readDir(ns: NS, path: string) {
  'use ls';
  const [server, ...directory] = path.split('/');

  const fs = ns.ls(server)
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

  return Object.keys(directory.reduce((prev, cur) => prev[cur], fs));
};

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

export function moveFile(ns: NS, source: string, destination: string) {
  const sourceExtension = source.split('.').at(-1);
  if (sourceExtension != 'js' && sourceExtension != 'txt') {
    throw new Error('only scripts and textfiles can be moved');
  }
  const [sourceServer, sourceFile] = source.split(/\/(.*)/, 2);
  const [destinationServer, destinationFile] = destination.split(/\/(.*)/, 2);

  console.log({
    source,
    sourceServer,
    sourceFile,

    destination,
    destinationServer,
    destinationFile,
  });

  if (sourceServer == destinationServer) ns.mv(sourceServer, sourceFile, destinationFile);
  else {
    copyFile(ns, source, destination);
    deleteFile(ns, source);
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
