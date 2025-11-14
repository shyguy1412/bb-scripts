export type DirEnt = { name: string, type: 'file' | 'folder'; };

type LsOptions = {
  withFileTypes: boolean;
};

export function list_directory(ns: NS, path: string, server: string, opts: { withFileTypes: true; }): DirEnt[];
export function list_directory(ns: NS, path: string, server?: string, opts?: { withFileTypes: false; }): string[];
export function list_directory(ns: NS, path: string, server = ns.self().server, opts?: LsOptions): DirEnt[] | string[] {
  const files = ns.ls(server).filter(p => p.startsWith(path))
    .map(f => ('/' + f).replace(new RegExp(`/?${path}/?([^/]*).*`), '$1'));

  if (opts?.withFileTypes) {
    return files.map(name => ({
      name,
      type: name.includes('.') ? 'file' : 'folder'
    } as DirEnt));
  }

  return files;
};

export function is_file(path: string) {
  return /\..?.?.?$/.test(path);
}

export function remove_directory(ns: NS, path: string, server = ns.self().server) {
  for (const file of ns.ls(server).filter(p => p.startsWith(path))) {
    ns.rm(file, server);
  }
}
