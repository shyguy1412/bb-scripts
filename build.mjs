import { context } from 'esbuild';
import { BitburnerPlugin } from 'esbuild-bitburner-plugin';
import fs from 'fs/promises';
import RamDodger3000 from 'ramdodger3000';

/** @type import('esbuild-bitburner-plugin').PluginExtension*/
const customExtension = {
  // afterConnect(remoteApi) {
  //   setInterval(async () => {
  //     const { result: files } = await remoteApi.getAllFiles('home');
  //     files
  //       .filter(file => file.filename.endsWith('.txt'))
  //       .forEach(file => writeFile(`logs/${file.filename}`, file.content));
  //   }, 500);
  // }
  async afterBuild() {
    const dodge = [
      'ServerManager.js'
    ];
    const output = await fs.readdir('./build', { recursive: true, withFileTypes: true })
      .then(f => f.filter(f => f.isFile() && dodge.includes(f.name)));

    await Promise.all(
      output.map(async file => fs.writeFile(
        `${file.path}/${file.name}`,
        RamDodger3000(await fs.readFile(`${file.path}/${file.name}`, { encoding: 'utf8' }))
      ))
    ).catch(_ => console.log(_));
  }
};

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


const createContext = async () => await context({
  entryPoints: [
    'src/servers/**/*.js',
    'src/servers/**/*.jsx',
    'src/servers/**/*.ts',
    'src/servers/**/*.tsx',
  ],
  outbase: './src/servers',
  outdir: './build',
  plugins: [
    CSSSpoofPlugin,
    SVGSpoofPlugin,
    BitburnerPlugin({
      port: 12525,
      types: 'NetscriptDefinitions.d.ts',
      extensions: [customExtension],
      mirror: {
        'mirror': ['home']
      },
      distribute: {
        'dist': ['grindr-1', 'grindr-2', 'grindr-3']
      },
      usePolling: true,
      pollingInterval: 100,
      // pushOnConnect: true,
    })
  ],
  bundle: true,
  format: 'esm',
  platform: 'browser',
  logLevel: 'debug',
});

let ctx = await createContext();
ctx.watch();

