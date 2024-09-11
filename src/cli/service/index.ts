import { cli } from "@/cli/service/cli";
import { parseService } from "@/cli/service/Parser";

export async function main(ns: NS) {
  console.clear();

  const service = ns.read('service.txt');

  try {
    console.log(parseService(service));
  } catch (e) { console.error(e); }

  return cli().parseAsync(ns).catch(e => ns.tprint(e.message));
  // //the filter filters out flags
  // const [command, ...input] = (ns.args as string[]).filter(([a]) => a != '-') as ValidCommands[];

  // if (!(command in Commands)) {
  //   ns.tprint(`Invalid command: ${command}`);
  //   return;
  // }

  // return Commands[command](ns, input).catch((e) => {
  //   ns.tprint('Something went unexpectedly wrong, sorry :(');
  //   console.error(e);
  // }).finally(() => ns.tprint('Done'));

}