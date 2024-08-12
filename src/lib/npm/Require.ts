import { GetServer } from '@/lib/exploit/Internals';
import { PackageJson } from '@/lib/npm/PackageParser';
import { ModuleDeclaration, Statement, parse as ast } from 'acorn';
import { simple } from 'acorn-walk';
import GlobToRegexp from 'glob-to-regexp';

export const PACKAGE_DIR = 'node_modules';

require.__cache = new Map<string, any>();
export async function require(path: string, cwd = `${PACKAGE_DIR}/${path}`): Promise<any> {
  const chacheHit = require.__cache.get(relativePathToAbsolute(path, cwd));
  if (chacheHit) return chacheHit;

  const entrypoint = resolve(path, cwd);

  const dependencyTree = parse(entrypoint);
  const { module } = await compile(dependencyTree);

  require.__cache.set(path, module);

  return module;
}

resolve.__cache = new Map<string, string>();
export function resolve(path: string, cwd: string): string {
  console.log(`RESOLVE: ${path}`, { path, cwd });
  const chacheHit = resolve.__cache.get(relativePathToAbsolute(path, cwd));
  if (chacheHit) return chacheHit;

  const entrypoint = resolveRelative(path, cwd) ?? //try to resolve as path
    resolveAlias(path, cwd) ?? //try to import as aliased module
    resolveModule(path); //try to import from installed modules

  if (!entrypoint) throw new Error(`Could not resolve ${path} from ${cwd}`);

  resolve.__cache.set(relativePathToAbsolute(path, cwd), entrypoint);

  return entrypoint;
}

function resolveRelative(path: string, cwd: string): string | null {
  if (!['/', './', '../'].some(prefix => path.startsWith(prefix))) return null;

  return relativePathToAbsolute(path, cwd);
}

function resolveAlias(path: string, cwd: string): string | null {
  if (!path.startsWith('#')) return null;

  const { imports, name } = findPackageJson(cwd);

  if (imports[path]) {
    const browserVariant = getBrowserVariant(imports[path]);
    if (!browserVariant) return null;
    console.log({ browserVariant, cwd });

    return relativePathToAbsolute(browserVariant, `${PACKAGE_DIR}/${name}`);
  };

  const alias = matchExportPattern(path, imports);

  if (!alias) return null;

  return relativePathToAbsolute(alias, cwd);
}

function resolveModule(path: string): string | null {
  const [module, submodule] = path.split('/');
  console.log({ module, submodule });

  const modulePath = `${PACKAGE_DIR}/${module}`;
  const { exports, main, name } = JSON.parse(load(`${modulePath}/package.json`)) as PackageJson;

  if (!exports) return relativePathToAbsolute(`./${main}`, modulePath);

  if (typeof exports == 'string') return relativePathToAbsolute(`./${exports}`, modulePath);

  if (!submodule) {
    const browserVariant = getBrowserVariant(exports['.']);
    if (!browserVariant) return null;
    console.log({ browserVariant, modulePath });

    return relativePathToAbsolute(browserVariant, modulePath);
  };

  return null;
}

function load(path: string): string {
  const contentFile = GetServer('home').getContentFile(path.replace(/\.[cm]js$/, '.js'));

  if (!contentFile) throw new Error('Could not load file: ' + path);

  return contentFile.content;
}

type DependencyTree = {
  path: string;
  content: string;
  name: string;
} & ({
  type: 'cjs';
} | {
  dependencies: DependencyTree[];
  type: 'esm';
});

parse.__cache = new Map<string, DependencyTree>;
export function parse(path: string): DependencyTree {
  const chacheHit = parse.__cache.get(path);
  if (chacheHit) return chacheHit;

  console.log('PARSE: ' + path);


  const [, dir, name] = path.split(/(.*)\//);
  const content = load(path);
  const program = ast(content, {
    ecmaVersion: 'latest',
    sourceType: 'module'
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

  if (!esm) return {
    path,
    name,
    content,
    type: 'cjs'
  };

  const imports: string[] = [];

  simple(program, {
    ImportDeclaration(node) {
      // Push this import onto the stack to replace
      if (!node.source) return;
      imports.push(node.source.value as string);
    },
    ExportNamedDeclaration(node) {
      if (!node.source) return;
      imports.push(node.source.value as string);
    },
    ExportAllDeclaration(node) {
      if (!node.source) return;
      imports.push(node.source.value as string);
    },
  });

  console.log({ dir, name, path });

  return {
    type: 'esm',
    dependencies: imports.map(path => parse(resolve(path, dir))),
    path: dir,
    name,
    content
  };
}

export async function compile(dependencyTree: DependencyTree): Promise<{ module: any; }> {
  console.log({ dependencyTree });

  if (dependencyTree.type == 'cjs') {
    const content = dependencyTree.content;
    const module = { exports: {} };
    const curriedRequire = (path: string) => require(path, dependencyTree.path);
    const moduleAsFunction = new Function('require', 'module', content);

    moduleAsFunction(curriedRequire, module);

    return { module: module.exports };
  }

  throw new Error('Compilation of ES Modules is not yet implemented, try importing the relevant entrypoint directly');

  return { module: undefined };
}

function matchExportPattern(path: string, patterns: PackageJson['imports']): string | null {
  return Object.keys(patterns)
    .reduce((prev: string | null, cur) => {
      const [prefix, suffix] = cur.split('*');
      if (path.startsWith(prefix) && path.endsWith(suffix)) {
        const replacer = path.replace(prefix, '').replace(suffix, '');
        const alias = getBrowserVariant(patterns[cur]);
        if (!alias) return prev;
        return alias.replace('*', replacer);
      }
      return prev;
    }, null);
}

function findPackageJson(path: string): PackageJson {

  const fragments = path.split('/');

  while (fragments.length) {
    try {
      const packageJson = load(`${fragments.join('/')}/package.json`);
      return JSON.parse(packageJson);
    } catch { }
    fragments.pop();
  }

  throw new Error('Could not find package.json: ' + path);
}

function getBrowserVariant(alias: PackageJson['imports'][string]): string | null {
  if (alias && typeof alias == 'string')
    return alias;
  else if (alias && typeof alias != 'string') {
    const browserVariant = (alias.browser ?? alias.default);
    if (typeof browserVariant == 'string') return browserVariant;
    return browserVariant.require ?? browserVariant.default;
  }

  return null;
}

function relativePathToAbsolute(path: string, cwd: string): string {
  const fragments = `${path.startsWith('./') ? cwd : ''}/${path}`.split('/').filter(f => f != '' && f != '.');
  const absolute: string[] = [];
  let skipCount = 0;
  for (let i = fragments.length - 1; i >= 0; i--) {
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
};
