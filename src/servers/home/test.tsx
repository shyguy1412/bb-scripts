import { FapComponents } from "@/lib/FapUI";
import { TUIGrid, TUIGridCell } from "@/lib/FapUI/tui/Grid";
import { CharCountContext, TUI } from "@/lib/FapUI/tui/TUIContainer";
import { useReload } from "@/lib/hooks/useReload";
import { createWindowApp } from "@/lib/WindowApp";
import { useContext } from "react";

export async function main(ns: NS) {
  const { Fragment } = FapComponents;

  console.clear();

  const windowApp = createWindowApp(ns);

  ns.atExit(() => windowApp.cleanup());

  return windowApp.mount(
    Fragment([
      () => (useReload(), null),
      TUI([
        TUIGrid([
          TUIGridCell(FillSpaceWith('#')),
          TUIGridCell(FillSpaceWith('X')),
          TUIGridCell(FillSpaceWith('O')),
        ])
          .SetLayout([
            ['a', 'a', '.'],
            ['b', 'c', 'c'],
            ['b', '.', '.'],
          ])
      ])
    ])
  );
}

function FillSpaceWith(name: string) {
  const { Div } = FapComponents;
  return () => {
    const { width, height } = useContext(CharCountContext);
    return Array(height).fill(null).map(_ => Div(Array(width).fill(name)));
  };
}