import { GetServer } from '@/lib/exploit/Internals';
import { PackageJson } from '@/lib/npm/PackageParser';
import { Plugin } from 'esbuild';
import esbuild from 'esbuild-wasm';

await esbuild.initialize({
  // wasmURL: esbuild_wasm_dataurl,
  wasmURL: 'https://www.unpkg.com/esbuild-wasm@0.23.0/esbuild.wasm',
  worker: true
});

export const PACKAGE_DIR = 'node_modules';

const ResolvePlugin: Plugin = {
  name: 'ResolvePlugin',
  setup(pluginBuild) {
    pluginBuild.onResolve({ filter: /.*/ }, (opts) => {
      const [, cwd] = opts.path.split(/(.*)\//);

      console.log(opts, { cwd });
      return {
        path: resolve(opts.path, cwd),
        namespace: 'bitburner'
      };
    });

    pluginBuild.onLoad({ namespace: 'bitburner', filter: /.*/ }, (opts) => {
      return {
        contents: load(opts.path)
      };
    });
  },
};

require.__cache = new Map<string, any>();
export async function require(path: string, cwd = `${PACKAGE_DIR}/${path}`): Promise<any> {
  const chacheHit = require.__cache.get(relativePathToAbsolute(path, cwd));
  if (chacheHit) return chacheHit;

  const { outputFiles: { 0: { contents: buffer } } } = await esbuild.build({
    entryPoints: [path],
    plugins: [ResolvePlugin],
    write: false,
    bundle: true,
    platform: 'browser',
    format: 'cjs',
    minify: true,
    keepNames: true,
    legalComments: 'none',
    logLevel: 'debug'
  });

  const moduleCode = new TextDecoder().decode(buffer);

  const module = compile(moduleCode);
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

  if (!imports) return null;

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
  const [module, submodule] = path.split(/\/(.*)/);
  console.log({ module, submodule });

  const modulePath = `${PACKAGE_DIR}/${module}`;
  const { exports, main, browser } = JSON.parse(load(`${modulePath}/package.json`)) as PackageJson;

  if (!exports) return relativePathToAbsolute(`./${browser ?? main}`, modulePath);

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

export function compile(content: string): any {
  const module = { exports: {} };
  const moduleAsFunction = new Function('require', 'module', 'exports', `${content};return exports`);

  const exports = moduleAsFunction(
    () => { throw new Error('Dynamic Require is not supported'); },
    module,
    {}
  );

  return Object.keys(exports).length ? exports : module.exports;
}

function matchExportPattern(path: string, patterns: NonNullable<PackageJson['imports']>): string | null {
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

function getBrowserVariant(alias: NonNullable<PackageJson['imports']>[string]): string | null {
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
