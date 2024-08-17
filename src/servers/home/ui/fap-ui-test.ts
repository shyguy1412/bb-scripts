import { createBinding, FapComponents } from '@/FapUI';
import { FapTable } from '@/FapUI/Table';

import _style from '@/css/FapTableTest.css' with {type: 'text'};
import { getPurchasedServers } from '@/lib/Network';
const style: string = _style as any; //TS typing shenanigans


export async function main(ns: NS) {
  ns.disableLog('ALL');
  ns.tail();
  ns.clearLog();
  console.clear();

  FragmentTest(ns);
  BindTest(ns);
  BasicTest(ns);
  TableTest(ns);

  ns.atExit(() => ns.clearLog());

  return new Promise(_ => { });
}

function FragmentTest(ns: NS) {
  const {
    Fragment, Span
  } = FapComponents;

  ns.printRaw(Fragment([
    Span(1), Span(2), Span(3)
  ]));
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

  ns.printRaw([
    Counter()
  ]);

}

function Counter() {
  const {
    Div
  } = FapComponents;

  const WrappedDiv = (count: number) => Div(count)
    .Style({ 'background': 'green' })
    .onClick(() => setCount(getCount() + 1));

  const [BoundDiv, [getCount, setCount]] = createBinding(WrappedDiv, 0);
  return BoundDiv;
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

  const TableWithClass = (...args: Parameters<typeof FapTable>) => FapTable(...args)
    .Class('fap-table')
    .CellPadding('5px')
    .onClick(() => setTranspose(!getTranspose()));

  const data = getServerData();
  const header = createHeader();

  const [ServerTable, [, setData], [, setHeader], [getTranspose, setTranspose]] = createBinding(TableWithClass, data, header, false);

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