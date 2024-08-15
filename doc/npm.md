# npm.js

## Set Alias

For the best experience alias npm.js to npm. `alias npm="run /npm.js"

## Download a package

This works the same way as it would with regular npm `npm <i/ci/install> <package>`.

## Import a package from your script

To import a package you will need to use the require function npm exports.

```js
import {require} from '/npm.js';

const jquery = await require('jquery');

export async function main(ns){
  const chalk = await require('chalk');
}
```

Require implements most of the node module resolution so it should behave very similiar to the nodejs native require function.

## Install a package programatically

npm.js also exports functions to interact with the npm registry and install packages programatically.

```js
import {getPackageInfo, installPackage} from '/npm.js';

export async function main(ns){
  const packageInfo = await getPackageInfo('axios', '1.7.4');
  await installPackage(ns, packageInfo);
}
```

## Not yet implemented

- `npm help`
- `npm outdated`
- `npm update`
- `npm remove`
- Dependency version verification (package-lock.json)
