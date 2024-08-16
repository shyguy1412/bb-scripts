import { createBinding, FapComponents, FapElement } from '@/lib/fap-ui';
import { FapTable } from '@/lib/fap-ui/Table';

import _style from '@/css/FapTableTest.css' with {type: 'text'};
import { getPurchasedServers } from '@/lib/Network';
const style: string = _style as any; //TS typing shenanigans


export async function main(ns: NS) {
  ns.disableLog('ALL');
  ns.tail();
  ns.clearLog();
  console.clear();

  // BindTest(ns);
  // BasicTest(ns);
  try {

    TableTest(ns);
  } catch (e) { console.error(e); }

  ns.atExit(() => ns.clearLog());

  return new Promise(_ => { });
}

function BasicTest(ns: NS) {

  const {
    Span, Button, Input, Br, Div
  } = FapComponents;

  const outputSpan = Span('').Style({ background: 'white', color: 'black', display: 'block' });

  ns.printRaw(

    Div([
      Button('Press me')
        .Style({ background: 'black', border: '1px solid red', color: 'red' })
        .onClick((el) => { el.Content('Ouchie!'); })
        .onMouseEnter((el) => { el.Content('Oh oh...'); })
        .onMouseLeave((el) => { el.Content('Press me'); }),
      ,
      Input().Id('my-input').Class('custom-class').Style({ display: 'block' }),
      Button('Submit').onClick(() => outputSpan.Content(['Input:', Br(), document.querySelector<HTMLInputElement>('#my-input')!.value])),
      outputSpan,
    ])

  );

}

function BindTest(ns: NS) {

  const [Bound, [getCount, setCount]] = createBinding(Counter, 0);

  ns.printRaw([
    Bound
  ]);

  const count = () => setTimeout(() => {
    ns.scan();
    setCount(getCount() + 1);
    console.log({ count: getCount });
    count();
  }, 1000);

  count();
}

function Counter(count: number) {
  const {
    Div
  } = FapComponents;

  return Div(count).Style({ 'background': 'green' });
}

function TableTest(ns: NS) {

  const {
    Style
  } = FapComponents;

  const getServerData = () => getPurchasedServers(ns).map(s => [
    ns.formatRam(s.maxRam), ns.formatRam(s.ramUsed), ns.formatRam(s.maxRam - s.ramUsed)
  ]);

  const createHeader = () => [
    ['Hostname', 'Max Ram', 'Used Ram', 'Free Ram'],
    getPurchasedServers(ns).map(s => s.hostname)
  ] as [string[], string[]];
  
  const TableWithClass = (...args:Parameters<typeof FapTable>) => FapTable(...args).Class('fap-table').CellPadding('5px')
  
    const data = getServerData();
    const header = createHeader();
  
  const [ServerTable, [, setData], [, setHeader]] = createBinding(TableWithClass, data, header, false);

  ns.printRaw([
    Style(style),
    ServerTable
  ]);

  const update = () => setTimeout(() => {
    setData(getServerData());
    setHeader(createHeader());
    update();
  }, 500);

  update();

}