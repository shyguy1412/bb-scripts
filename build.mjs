import { context } from 'esbuild';
import { BitburnerPlugin } from 'esbuild-bitburner-plugin';
import { OverloadPlugin } from 'esbuild-operator-overload-plugin';
import fs from 'fs/promises';
import { RamDodgerExtension, UnsafePlugin } from 'ramdodger-extension';

/**
 * @type {import('esbuild').Plugin}
 */
const CSSSpoofPlugin = {
  name: 'CSSSpoofPlugin',
  setup(pluginBuild) {
    pluginBuild.onLoad({ filter: /.*?\.css$/ }, async opts => {
      const file = await fs.readFile(opts.path, { encoding: 'utf8' });
      return {
        loader: 'jsx',
        contents: `\
        import React from 'react';

        export default function () {
          return <style>{\`${file}\`}</style>;
        }\
        `
      };
    });
  }
};

/**
 * @type {import('esbuild').Plugin}
 */
const SVGSpoofPlugin = {
  name: 'SVGSpoofPlugin',
  setup(pluginBuild) {
    pluginBuild.onLoad({ filter: /.*?\.svg$/ }, async opts => {
      const file = await fs.readFile(opts.path, { encoding: 'utf8' });
      return {
        loader: 'jsx',
        contents: `\
        import React from 'react';

        export default function () {
          return <div dangerouslySetInnerHTML={{__html: \`${file}\`}}></div>;
        }\
        `
      };
    });
  }
};


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


const createContext = async () => await context({
  entryPoints: [
    'src/servers/**/*.js',
    'src/servers/**/*.jsx',
    'src/servers/**/*.ts',
    'src/servers/**/*.tsx',
  ],
  // entryPoints: ['src/servers/grindr-1/test.ts'],
  outbase: './src/servers',
  outdir: './build',
  plugins: [
    CSSSpoofPlugin,
    SVGSpoofPlugin,
    UnsafePlugin,
    // OverloadPlugin,
    BitburnerPlugin({
      port: 12525,
      types: 'NetscriptDefinitions.d.ts',
      extensions: [RamDodgerExtension],
      mirror: {
        'mirror/own': 'own',
        'mirror/all': 'all',
        'mirror/other': 'other'
      },
      distribute: {
        'build/all': 'all',
        'all': 'all',
      },
      // usePolling: true,
      // pollingInterval: 100,
      // pushOnConnect: true,
    })
  ],
  bundle: true,
  format: 'esm',
  platform: 'browser',
  // minify: true,
  keepNames: true,
  logLevel: 'debug',
});

const ctx = await createContext();
ctx.watch();
// await ctx.rebuild();
// ctx.dispose();