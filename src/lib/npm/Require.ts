import { GetServer } from '@/lib/exploit/Internals';
import { PACKAGE_DIR } from '@/lib/npm/cli';
import { ModuleDeclaration, Statement, parse as ast } from 'acorn';


require.__cache = new Map<string, any>();
export function require(path: string): any {
  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  //@ts-expect-error: overwrite require
  window._require = window.require;
  //@ts-expect-error: overwrite require
  window.require = require;

  const entrypoint = resolve(path);
  console.log(entrypoint);

  const dependencyTree = parse(entrypoint);
  const { module } = compile(dependencyTree);
  require.__cache.set(path, module);

  //@ts-expect-error: reset require
  window.require = window._require;
  //@ts-expect-error: reset require
  delete window._require;
  return module;
}

resolve.__cache = new Map<string, string>();
export function resolve(path: string, cwd?: string): string {

  if (path.startsWith('.') && cwd) return relativePathToAbsolute(path, cwd);
  else if (path.startsWith('.')) throw new Error("resolve: cwd must be set for relative paths");

  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  const server = GetServer('home');
  server.getContentFile = server.getContentFile.bind(server);
  const { getContentFile } = server;

  const packagePath = `${PACKAGE_DIR}/${path}`;
  const packageJson = JSON.parse(getContentFile(`${packagePath}/package.json`).text ?? 'null');

  if (!packageJson) throw new Error(`resolve: resolving ${path} failed, could not find package.json`);

  const entrypoint = `${packagePath}/${packageJson.main}`;

  resolve.__cache.set(path, entrypoint);

  return entrypoint;
}

function load(path: string) {
  return GetServer('home').getContentFile(path).content;
}

type DependencyTree = {
  [path: string]: {
    content: string;
    dependencies: DependencyTree;
  };
};

parse.__cache = new Map<string, DependencyTree>;
export function parse(path: string): DependencyTree {
  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  const content = load(path);
  const program = ast(content, {
    ecmaVersion: 'latest'
  });

  const esm = program.body.some(
    statement => ([
      "ExportAllDeclaration",
      "ExportDefaultDeclaration",
      "ExportNamedDeclaration",
      "ImportDeclaration"
    ] as (Statement | ModuleDeclaration)['type'][])
      .includes(statement.type)
  );

  if (esm) throw new Error("Require of esm modules is not implemented. Instead import from this path: " + path);


  return {
    [path]: {
      content,
      dependencies: {}
    }
  };
}

export function compile(dependencyTree: DependencyTree): { module: any; } {
  const content = Object.values(dependencyTree)[0].content;
  const module = { exports: {} };
  {
    eval(content);
  }
  return { module: module.exports };
}


function relativePathToAbsolute(path: string, cwd: string): string {
  const fragments = (cwd + '/' + path).split('/').filter(f => f != '' && f != '.');
  const absolute: string[] = [];
  let skipCount = 0;
  for (let i = fragments.length - 1; i > 0; i--) {
    const fragment = fragments[i];

    if (fragment == '..' && fragments[i - 1] == '..') {
      skipCount++;
      continue;
    }

    if (fragment == '..' && fragments[i - 1] != '..') {
      i -= skipCount + 1;
      skipCount = 0;
      continue;
    }

    absolute.unshift(fragment);
  }
  return absolute.join('/');
}
