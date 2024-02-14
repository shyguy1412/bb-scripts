import { generate } from 'astring';
import { parse } from 'acorn';
import { ancestor } from 'acorn-walk';
import { context, transform } from 'esbuild';
import { BitburnerPlugin } from 'esbuild-bitburner-plugin';
import fs from 'fs/promises';
import RamDodger3000 from 'ramdodger3000';

/** @type import('esbuild-bitburner-plugin').PluginExtension*/
const RamDodgerExtension = {
  async afterBuild() {
    const output = await fs.readdir('./build', { recursive: true, withFileTypes: true })
      .then(f => f.filter(f => f.isFile()));

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

/**
 * @type {import('esbuild').Plugin}
 */
const UnsafePlugin = {
  name: 'UnsafePlugin',
  setup(pluginBuild) {

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == 'unsafe') {
        const file = await fs.readFile(opts.path, { encoding: 'utf8' });
        const { code } = await transform(file, { loader: 'tsx' });
        const ast = parse(code, { ecmaVersion: 'latest', sourceType: 'module' });

        ancestor(ast, {
          ExpressionStatement(node, state) {
            if (node.directive) {
              const parent = state.at(-2);
              parent.body = parent.body.filter(node => !node.directive);
            }
          }
        });

        return {
          contents: generate(ast)
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
  outbase: './src/servers',
  outdir: './build',
  plugins: [
    CSSSpoofPlugin,
    SVGSpoofPlugin,
    UnsafePlugin,
    BitburnerPlugin({
      port: 12525,
      types: 'NetscriptDefinitions.d.ts',
      extensions: [RamDodgerExtension],
      mirror: {
        'mirror': 'all'
      },
      distribute: {
        'build/all': 'all'
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

const ctx = await createContext();
ctx.watch();