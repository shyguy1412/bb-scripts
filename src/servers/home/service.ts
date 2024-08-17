import { Commands, ValidCommands } from "@/service/cli";
import { parseService } from "@/service/Parser";

export async function main(ns: NS) {
  console.clear();

  const service = ns.read('service.txt');

  try {
    console.log(parseService(service));
  } catch (e) { console.error(e); }


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