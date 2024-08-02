# Shyguys Bitburner scripts

This repo uses the [bb-external-editor](https://github.com/shyguy1412/bb-external-editor) template!
Also check out my [plugin](https://github.com/NilsRamstoeck/esbuild-bitburner-plugin) that powers all this

Transpiled and stand-alone versions of my scripts are available under releases.

Notice: Most my scripts dodge ram by offloading ram heavy operations to a worker script. This worker script is called `ram-allocator.js` and will be generated automatically if it doesnt exist. To ensure everything works fine avoid deleting or modifying this file.

## Currently Maintained Scripts

- [Dolphin](./doc/Dolphin.md) A simple file explorer with drag/drop support
- [Konsole](./doc/Konsole.md) Clones the terminal into a tail window (for use with Plasma)
- [Plasma](./doc/Plasma.md) A KDE Plasma inspired Desktop Enviroment
- [WindowApp](./doc/WindowApp.md) A library for making UI apps with react utilizing the tail windows.
- [ServerManager](./doc/ServerManager.md) UI tool to buy, upgrade and delete servers
- [ServerViewer](./doc/ServerViewer.md) View stats of servers on the network.
- [Batcher](./doc/Batcher.md) My batcher. Works with and without Formulas API.
- [RootGainer](./doc/RootGainer.md) Runs in the background until every server in the network is rooted.
