import { GetServer } from '@/lib/exploit/Internals';
import { PackageJson } from '@/lib/npm/PackageParser';
import { ModuleDeclaration, Statement, parse as ast } from 'acorn';
import { simple } from 'acorn-walk';

export const PACKAGE_DIR = 'node_modules';

require.__cache = new Map<string, any>();
export async function require(path: string, from?: string): Promise<any> {
  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  console.log(`REQUIRE: ${path} ${from ? `from ${from}` : ''}`);


  const cwd = from ? from : `${PACKAGE_DIR}/${path}`;
  const packageJson = from ?
    getPackageJsonFromPath(from) :
    JSON.parse(load(`${cwd}/package.json`) ?? 'null');

  if (!packageJson) throw new Error('Required module is not part of a package');

  const entrypoint = resolve(path, cwd, packageJson);

  const dependencyTree = parse(entrypoint, cwd, packageJson);
  const { module } = await compile(dependencyTree);
  require.__cache.set(path, module);

  return module;
}

resolve.__cache = new Map<string, string>();
export function resolve(path: string, cwd: string, packageJson: PackageJson): string {
  console.log(`RESOLVE: ${path}`, { path, cwd, packageJson });

  if (path.startsWith('.') && cwd) return relativePathToAbsolute(path, cwd);

  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  const { exports } = packageJson;


  if (!exports || typeof exports == 'string') {
    const entrypoint = relativePathToAbsolute(exports ?? packageJson.main, cwd);
    resolve.__cache.set(path, entrypoint);
    return entrypoint;
  }


  throw new Error(`Could not resolve ${path} from ${cwd}`);
}

function load(path: string): string {
  const contentFile = GetServer('home').getContentFile(path);

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
export function parse(path: string, cwd: string, packageJson: PackageJson): DependencyTree {
  const chacheHit = require.__cache.get(path);
  if (chacheHit) return chacheHit;

  const [dir, name] = path.split(/\/(.*)/, 2);
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


  return {
    type: 'esm',
    dependencies: imports.map(path => parse(resolve(path, cwd, packageJson), cwd, packageJson)),
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
    const _require = require; //scoping shenanigans
    {
      //curry require to include path of the currently executing file
      const require = (path: string) => _require(path, dependencyTree.path);
      eval(content);
    }
    return { module: module.exports };
  }

  return { module: undefined };
}


function relativePathToAbsolute(path: string, cwd: string): string {
  const fragments = (cwd + '/' + path).split('/').filter(f => f != '' && f != '.');
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
}

function getPackageJsonFromPath(path: string): PackageJson | undefined {
  const fragments = path.split('/');
  while (fragments.length) {
    const path = fragments.join('/');
    try {
      const packageJson = JSON.parse(load(path));
      return packageJson;
    } catch { }
    fragments.pop();
  }
  return;
}