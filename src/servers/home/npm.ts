import { Commands, ValidCommands } from "@/lib/npm/cli";

export async function main(ns: NS) {
  console.clear();

  //the filter filters out flags
  const [command, ...input] = (ns.args as string[]).filter(([a]) => a != '-') as ValidCommands[];

  if (!(command in Commands)) {
    ns.tprint(`Invalid command: ${command}`);
    return;
  } 

  return Commands[command](ns, ...input).catch((e) => {
    ns.tprint('Something went unexpectedly wrong, sorry :(');
    console.error(e);
  }).finally(() => ns.tprint('Done'));

}

export { installPackage } from '@/lib/npm/cli';
export * from '@/lib/npm/ApiWrapper';
export { require } from '@/lib/npm/Require';