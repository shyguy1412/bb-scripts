import { context } from 'esbuild';
import { BitburnerPlugin } from 'esbuild-bitburner-plugin';
import fs from 'fs/promises';
import { UnsafePlugin } from 'ramdodger-extension';

/**
 * @type {import('esbuild').Plugin}
 */
const CustomImportAttributes = {
  name: 'CustomImportAttributes',
  setup(pluginBuild) {

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'css') {
        return {
          contents: `import css from '${opts.path}' with {type: 'text'};const sheet = new CSSStyleSheet();await sheet.replace(css);export default sheet;`,
          loader: 'js'
        };
      }
    });

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'text') {
        const file = await fs.readFile(opts.path, { encoding: 'utf8' });
        return {
          contents: file,
          loader: 'text'
        };
      }
    });

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'dataurl') {
        const file = await fs.readFile(opts.path, { encoding: 'utf8' });
        return {
          contents: file,
          loader: 'dataurl'
        };
      }
    });

  }
};

/**
 * @type {import('esbuild').Plugin}
 */
const MetaImports = {
  name: 'MetaImports',
  setup(pluginBuild) {

    pluginBuild.onResolve({ filter: /meta/ }, (opts) => {
      return {
        path: opts.importer,
        pluginData: opts,
        namespace: opts.path
      };
    });

    pluginBuild.onLoad({ filter: /.*/, namespace: 'meta:filename' }, async (opts) => {
      return {
        contents: opts.path.replace(/^.*\/(.*)\..*$/, '$1'),
        loader: 'text'
      };
    });

  }
};

/**@type {import('esbuild-bitburner-plugin').PluginExtension} */
const ClearChunksExtension = {
  async afterBuild(rfa) {
    const files = (await rfa.getFileNames('home')).result;
    const chunks = files.filter(f => f.startsWith('usr/lib/chunks'));
    const bin = files.filter(f => f.startsWith('bin'));
    await Promise.all(chunks.map(c => rfa.deleteFile({ filename: c, server: 'home' }))
    );
  }
};

/** @type {import('esbuild').BuildOptions} */
export const config = {
  entryPoints: [
    'src/servers/**/*.js',
    // 'src/servers/**/*.jsx',
    'src/servers/**/*.ts',
    'src/servers/**/*.tsx',
    // 'src/servers/home/test.txt',
    // 'src/servers/home/rust/example/src/lib.rs'
  ],
  // entryPoints: ['src/servers/grindr-1/test.ts'],
  outbase: './src/servers',
  outdir: './build',
  plugins: [
    CustomImportAttributes,
    MetaImports,
    // SVGSpoofPlugin,
    UnsafePlugin,
    BitburnerPlugin({
      port: 12525,
      types: 'NetscriptDefinitions.d.ts',
      mirror: { 'mirror': ['home'] },
      extensions: [ClearChunksExtension]
    })
  ],
  bundle: true,
  format: 'esm',
  platform: 'browser',
  // sourcesContent: true,
  // minify: true,
  keepNames: true,
  logLevel: 'debug',
  // splitting: true,
  chunkNames: 'home/usr/lib/chunks/[name]-[hash]'
};


if (import.meta.filename == process.argv[1]) {
  const ctx = await context(config);
  ctx.watch();
}