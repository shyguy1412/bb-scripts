# Commander.js

Commander.js is a port of the cli framework [Commander.js](https://www.npmjs.com/package/commander). View their Github or NPM package for a full documentation.

The only thing that has been added to Commander.js are the `suggest` and `autocomplete` methods.  
With `suggest` you can pass an array of suggestions for any argument or option value. Then you can use `autocomplete` to easily generate the autocompletion for all arguments, options and sub-commands.

Here is an example tool that generates a table of servers and their values

```js
import { createCommand } from "/Commander.js";
import { FapComponents } from "/FapUI.js";

//list of all props a server object can have
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
];

//generates an ASCII table with some given data
export function AsciiTable(ns, data) {
  const valueFormatter = (val, i) => {
    if (val == data[0][i]) return val.replace(/[A-Z]/g, (v) => ` ${v.toLocaleLowerCase()}`);

    if (data[0][i].toLocaleLowerCase().includes('ram')) {
      return ns.formatRam(+val);
    }

    return val;
  };

  const formattedData = data.map(c => c.map(valueFormatter));
  const colWidths = formattedData.reduce((prev, row) => prev.map((v, i) => row[i].length > v ? row[i].length : v), formattedData[0].map(v => v.length));
  const table = formattedData.reduce((prev, cur, i, { length }) => {

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
  const command = createCommand() //create a command
    .description('Get formatted data for a list of servers')
    .argument('servers...', 'list of servers to get data for')
    .option('-i, --include', 'only include columns that have been listed in the filter. this is the default setting')
    .option('-x, --exclude', 'exclude columns that have been listed in the filter.')
    .option('-c, --columns [columns...]', 'ordered list of columns to display')
    .action((ns, servers, options) => { //this function will be called if this command was executed

      const { include, exclude, columns } = options;

      if (include && exclude) return command.error('Error: cannot set include and exclude flag at the same time');

      const filter = columns && columns.length ? columns : SERVER_PROPS;
      const filteredColumns = columns.filter(el => exclude ? !filter.includes(el) : filter.includes(el));

      const data = servers.map(s => ns.getServer(s)).map(s => filteredColumns.map(c => s[c] + ''));

      const { Div } = FapComponents; //Use FapUI to easily render styled HTML
      ns.tprintRaw(
        Div(AsciiTable(ns, [filteredColumns, ...data])).Style({ lineHeight: 1 })
      );

    });
  return command;
}

export function autocomplete(data, args) {
  return cli()
    .suggest('servers', data.servers) //suggest all servers for the 'servers' attribute
    .suggest('columns', SERVER_PROPS) //suggest all props for the 'columns' attribute
    .autocomplete(data, args);
}

export async function main(ns) {
  return cli().parse(ns); //this will parse all arguments and call the correct command
}
```
