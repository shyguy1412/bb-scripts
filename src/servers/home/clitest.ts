import { program } from "@/lib/Commander";
import { AutocompleteData } from "NetscriptDefinitions";

function cli() {
  const command = program()
    .description('Split a string into substrings and display as an array')
    .argument('<string>', 'string to split')
    .option('--first', 'display just the first substring')
    .option('-s, --separator <char>', 'separator character', ',')
    .action((ns, str, options) => {
      const limit = options.first ? 1 : undefined;
      ns.tprint(str.split(options.separator, limit));
    });

  return command;
}

export function autocomplete(data: AutocompleteData, args: string[]) {
  console.clear();
  return cli().autocomplete(data, args);
}

export async function main(ns: NS) {
  return cli().parse(ns);
}