import { createCommand } from "@/lib/Commander";
import { AutocompleteData, Server } from "NetscriptDefinitions";
import { FapComponents } from "@/FapUI";

const { Div } = FapComponents;

const SERVER_PROPS = [
  'hostname',
  'ip',
  'sshPortOpen',
  'ftpPortOpen',
  'smtpPortOpen',
  'httpPortOpen',
  'sqlPortOpen',
  'hasAdminRights',
  'cpuCores',
  'isConnectedTo',
  'ramUsed',
  'maxRam',
  'organizationName',
  'purchasedByPlayer',
  'backdoorInstalled',
  'baseDifficulty',
  'hackDifficulty',
  'minDifficulty',
  'moneyAvailable',
  'moneyMax',
  'numOpenPortsRequired',
  'openPortCount',
  'requiredHackingSkill',
  'serverGrowth',
] as readonly string[];

export function AsciiTable(data: readonly string[][]) {

  const colWidths = data.reduce((prev, row) => prev.map((v, i) => row[i].length > v ? row[i].length : v), data[0].map(v => v.length));
  const table = data.reduce((prev, cur, i, { length }) => {

    const filler = colWidths.map(w => Array(w).fill(i && i != length - 1 ? '─' : '═').join('')).join(i && i != length - 1 ? '┼' : !i ? '╪' : '╧');

    let row = '';

    if (i == 0) {
      row += '╔' + filler.replaceAll('╪', '╤') + '╗' + '\n';
    }

    row += '║' + cur.map((v, i) => v.padEnd(colWidths[i], ' ')).join('│') + '║' + '\n';

    if (i == 0)
      row += '╠' + filler + '╣';
    else if (i != length - 1)
      row += '╟' + filler + '╢';
    else
      row += '╚' + filler + '╝';

    return prev + '\n' + row;
  }, '');


  return table;
}

function cli() {
  const command = createCommand()
    .description('Get formatted data for a list of servers')
    .argument('servers...', 'list of servers to get data for')
    .option('-i, --include', 'only include columns that have been listed in the filter. this is the default setting')
    .option('-x, --exclude', 'exclude columns that have been listed in the filter.')
    .option('-c, --columns [columns...]', 'ordered list of columns to display')
    .action((ns, servers, options) => {

      const { include, exclude, columns } = options;

      if (include && exclude) return command.error('Error: cannot set include and exclude flag at the same time');

      const filter = columns && columns.length ? columns as string[] : SERVER_PROPS;
      const filteredColumns = (columns as string[]).filter(el => exclude ? !filter.includes(el) : filter.includes(el));

      const valueFormatter = (row: string[], i: number, data: string[][]) => {
        if (i == 0) return row.map(v => v.replace(/[A-Z]/g, (v) => ` ${v.toLocaleLowerCase()}`));

        return row.map((v, i) => {
          if (data[0][i].toLocaleLowerCase().includes('ram')) {
            return ns.formatRam(+v);
          }

          if (data[0][i].toLocaleLowerCase().includes('money')) {
            return ns.formatNumber(+v);
          }
          return v;
        });
      };

      const data = (servers as string[])
        .map(s => ns.getServer(s))
        .map(s => filteredColumns.map(c => s[c as keyof Server] + ''));

      data.unshift(filteredColumns);

      ns.tprintRaw(
        Div(AsciiTable(data.map(valueFormatter))).Style({ lineHeight: 1 })
      );

    });
  return command;
}

export function autocomplete(data: AutocompleteData, args: string[]) {
  return cli()
    .suggest('servers', data.servers)
    .suggest('columns', SERVER_PROPS)
    .autocomplete(data, args);
}

export async function main(ns: NS) {
  return cli().parse(ns);
}