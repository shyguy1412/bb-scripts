import { context } from 'esbuild';
import { BitburnerPlugin } from 'esbuild-bitburner-plugin';
import fs from 'fs/promises';
import { UnsafePlugin } from 'ramdodger-extension';

/**
 * @type {import('esbuild').Plugin}
 */
const CSSPlugin = {
  name: 'CSSPlugin',
  setup(pluginBuild) {
    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'css') {
        return {
          contents: `import css from '${opts.path}' with {type: 'text'};const sheet = new CSSStyleSheet();await sheet.replace(css);export default sheet;`,
          loader: 'js'
        };
      }

    });
  }
};

// /**
//  * @type {import('esbuild').Plugin}
//  */
// const SVGSpoofPlugin = {
//   name: 'SVGSpoofPlugin',
//   setup(pluginBuild) {
//     pluginBuild.onLoad({ filter: /.*?\.svg$/ }, async opts => {
//       const file = await fs.readFile(opts.path, { encoding: 'utf8' });
//       return {
//         loader: 'jsx',
//         contents: `\
//         import React from 'react';

//         export default function () {
//           return <div dangerouslySetInnerHTML={{__html: \`${file}\`}}></div>;
//         }\
//         `
//       };
//     });
//   }
// };


/**
 * @type {import('esbuild').Plugin}
 */
const TextPlugin = {
  name: 'TextPlugin',
  setup(pluginBuild) {

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'text') {
        const file = await fs.readFile(opts.path, { encoding: 'utf8' });
        return {
          contents: file,
          loader: 'text'
        };
      }

    });
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
    TextPlugin,
    CSSPlugin,
    // SVGSpoofPlugin,
    UnsafePlugin,
    BitburnerPlugin({
      port: 12525,
      types: 'NetscriptDefinitions.d.ts',
      mirror: { 'mirror': ['home'] }
    })
  ],
  bundle: true,
  format: 'esm',
  platform: 'browser',
  // sourcesContent: true,
  // minify: true,
  keepNames: true,
  logLevel: 'debug',
};


if (import.meta.filename == process.argv[1]) {
  const ctx = await context(config);
  ctx.watch();
}