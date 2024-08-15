import { FapComponents } from "@/lib/fap-ui";

const {
  Button, Input, Div, Span, Br
} = FapComponents;

export async function main(ns: NS) {
  ns.disableLog('ALL');
  ns.tail();
  ns.clearLog();

  const outputSpan = Span("").Style({ background: 'white', color: 'black', display: 'block' });

  ns.printRaw(

    Div([
      Button('Press me')
        .Style({ background: 'black', border: '1px solid red', color: 'red' })
        .onClick((el) => { el.Content('Ouchie!'); })
        .onMouseEnter((el) => { el.Content('Oh oh...'); })
        .onMouseLeave((el) => { el.Content('Press me'); }),
      ,
      Input().Id('my-input').Class('custom-class').AriaBrailleroledescription('brrr').DataStuff('brrr').Style({ display: 'block' }),
      Button('Submit').onClick(() => outputSpan.Content(["Input:", Br(), document.querySelector<HTMLInputElement>('#my-input')!.value])),
      outputSpan,
    ])

  );

  ns.atExit(() => ns.clearLog());

  return new Promise(_ => { });
}