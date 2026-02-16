import { BuildOptions, context, Plugin } from "esbuild";
import { BitburnerPlugin, PluginExtension } from "esbuild-bitburner-plugin";

const CustomImportAttributes: Plugin = {
  name: "CustomImportAttributes",
  setup(pluginBuild) {
    pluginBuild.onLoad({ filter: /.*/ }, (opts) => {
      if (opts.with.type == "css") {
        return {
          contents:
            `import css from '${opts.path}' with {type: 'text'};const sheet = new CSSStyleSheet();await sheet.replace(css);export default sheet;`,
          loader: "js",
        };
      }
    });

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == "text") {
        const file = await Deno.readTextFile(opts.path);
        return {
          contents: file,
          loader: "text",
        };
      }
    });

    pluginBuild.onLoad({ filter: /.*/ }, async (opts) => {
      if (opts.with.type == "dataurl") {
        const file = await Deno.readTextFile(opts.path);
        return {
          contents: file,
          loader: "dataurl",
        };
      }
    });
  },
};

const MetaImports: Plugin = {
  name: "MetaImports",
  setup(pluginBuild) {
    pluginBuild.onResolve({ filter: /meta/ }, (opts) => {
      return {
        path: opts.importer,
        pluginData: opts,
        namespace: opts.path,
      };
    });

    pluginBuild.onLoad(
      { filter: /.*/, namespace: "meta:filename" },
      (opts) => {
        return {
          contents: opts.path.replace(/^.*\/(.*)\..*$/, "$1"),
          loader: "text",
        };
      },
    );
  },
};

const ClearChunksExtension: PluginExtension = {
  async afterBuild(rfa) {
    const files = (await rfa.getFileNames("home")).map(r => r.result).unwrapOr([]) as string[];
    const chunks = files.filter((f) => f.startsWith("usr/lib/chunks"));
    // const bin = files.filter((f) => f.startsWith("bin"));
    await Promise.all(
      chunks.map((c) => rfa.deleteFile({ filename: c, server: "home" })),
    );
  },
};

export const config: BuildOptions = {
  entryPoints: [
    // "src/home/**/*.js",
    // 'src/servers/**/*.jsx',
    "src/home/**/*.ts",
    "src/home/**/*.tsx",
    // 'src/servers/home/test.txt',
    // 'src/servers/home/rust/example/src/lib.rs'
  ],
  // entryPoints: ['src/servers/grindr-1/test.ts'],
  outbase: "./src/home",
  outdir: "./build",
  plugins: [
    CustomImportAttributes,
    MetaImports,
    // SVGSpoofPlugin,
    // UnsafePlugin,
    BitburnerPlugin({
      port: 12525,
      types: "NetscriptDefinitions.d.ts",
      mirror: { "mirror": ["home"] },
      extensions: [ClearChunksExtension],
    }),
  ],
  bundle: true,
  format: "esm",
  platform: "browser",
  // sourcesContent: true,
  // minify: true,
  keepNames: true,
  logLevel: "info",
  // splitting: true,
  chunkNames: "home/usr/lib/chunks/[name]-[hash]",
};

if (import.meta.main) {
  const ctx = await context(config);
  ctx.watch();
}
