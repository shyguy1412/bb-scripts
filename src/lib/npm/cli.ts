import { PackageInfo, getPackage, getPackageInfo } from "@/lib/npm/ApiWrapper";
import { ScriptArg } from "NetscriptDefinitions";

export const PACKAGE_DIR = 'node_modules';

export type ValidCommands =
  'i' | 'ci' | 'install' |
  'r' | 'remove' |
  'outdated' | 'update' |
  'help';

export const Commands: { [key in ValidCommands]: (ns: NS, ...input: ScriptArg[]) => Promise<void> } = {
  i(ns, ...input) {
    return this.install(ns, ...input);
  },
  ci(ns, ...input) {

    return this.install(ns, ...input);
  },
  async install(ns, ...input) {
    const [requestedPackage] = input;

    if (typeof requestedPackage != 'string') {
      ns.tprint(`Error: expected package name to be string but was ${typeof requestedPackage}`);
      return;
    }

    const [packageName, packageVersion] = requestedPackage.split('@');

    const packageInfo = await getPackageInfo(packageName, packageVersion).catch(({ error }) => {
      ns.tprint(error);
    });

    if (!packageInfo) return;

    return installPackage(ns, packageInfo);

  },
  r(ns, ...input) {
    return this.remove(ns, ...input);
  },
  remove(ns, ...input) {
    throw new Error("Function not implemented.");
  },
  outdated(ns, ...input) {
    throw new Error("Function not implemented.");
  },
  update(ns, ...input) {
    throw new Error("Function not implemented.");
  },
  help(ns, ...input) {
    throw new Error("Function not implemented.");
  }
};

export async function installPackage(ns: NS, packageInfo: PackageInfo, visited: string[] = []) {
  const files = await getPackage(packageInfo);

  for (const { name, content } of files) {
    ns.write(`node_modules/${packageInfo.name}/${name}`, content, 'w');
  }

  visited.push(packageInfo.name);

  await Promise.all(Object.keys(packageInfo.dependencies??{})
    .map(async dep => (installPackage(ns, await getPackageInfo(dep), visited)))
  );

}