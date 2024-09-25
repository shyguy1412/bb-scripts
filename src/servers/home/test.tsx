import { FapComponents, FapWrap, useFap } from "@/lib/FapUI";
import { TUIGrid } from "@/lib/FapUI/tui/TUIGrid";
import { CharCountContext, TUI } from "@/lib/FapUI/tui/TUIContainer";
import { useReload } from "@/lib/hooks/useReload";
import { createWindowApp } from "@/lib/WindowApp";
import { useContext } from "react";
import { TUIButton } from "@/lib/FapUI/tui/TUIButton";

export async function main(ns: NS) {
  try {

    const { Fragment } = FapComponents;

    console.clear();

    const windowApp = createWindowApp(ns);

    ns.atExit(() => windowApp.cleanup());

    const label = 'Click Me!';

    const testbutton = () => TUIButton(label)
      .onMouseOver((el) => {
        el.Style({ background: 'var(--primary)', color: 'var(--backgroundprimary)', cursor: 'pointer' });
      })
      .onMouseOut((el) => {
        el.Style({});
      })
      .onMouseDown((el) => {
        el.Style({ cursor: 'pointer' });
      })
      .onMouseUp((el) => {
        el.Style({ background: 'var(--primary)', color: 'var(--backgroundprimary)', cursor: 'pointer' });
      })
      .onClick((el) => {
        el.Content('Clicked!');
      });

    return windowApp.mount(
      Fragment([
        () => (useReload(), null),
        TUI([
          TUIGrid([
            [
                testbutton
            ],
            FillSpaceWith('X'),
            FillSpaceWith('O'),
          ])
            .SetLayout([
              ['a', 'a', '.'],
              ['b', 'c', 'c'],
              ['b', '.', '.'],
            ])
        ])
      ])
    );
  } catch (e) { console.error(e); }
}

function FillSpaceWith(name: string) {
  return () => {
    const { width, height } = useContext(CharCountContext);
    return Array(width * height).fill(name);//.map(_ => Div(Array(width).fill(name)));
  };
}