import { FapComponents, FapContent, FapElement, FapWrap, recursivePrepareContent, useFap } from "@/lib/FapUI";
import { CharCountContext, EmContext } from "@/lib/FapUI/tui/TUIContainer";
import React, { useMemo } from "react";
import { useContext } from "react";

export function TUIGrid(cells: TUIGridCell[]) {

  const { state, bind } = useFap({
    areas: [[]] as string[][]
  });

  const component = () => {

    const { areas } = bind();

    const charCount = useContext(CharCountContext);
    const emSize = useContext(EmContext);

    const newCharCount = useMemo(() => ({
      width: Math.floor(charCount.width / areas.length) * areas.length,
      height: Math.floor(charCount.height / areas[0].length) * areas.length,
    }), [areas, charCount]);

    console.log({ areas });

    const slots = useMemo(() => {
      const slots: Record<string, {
        name: string,
        pos: { x: number, y: number; };
        end: { x: number, y: number; };
        content: TUIGridCell | null;
      }> = {};
      const cellsCopy = [...cells];
      for (let y = 1; y <= areas.length; y++) {
        for (let x = 1; x <= areas[y - 1].length; x++) {
          const name = areas[y - 1][x - 1];
          if (name == '.') continue;
          if (!slots[name]) {
            slots[name] = {
              name,
              pos: { x, y },
              end: { x, y },
              content: cells.shift() ?? null
            };
          }
          slots[name].end.x = Math.max(slots[name].end.x, x);
          slots[name].end.y = Math.max(slots[name].end.y, y);
        }
      }
      return Object.values(slots);
    }, [areas]);

    console.log({slots, newCharCount});

    const els = [
      {
        name: '#',
        pos: { x: 1, y: 1 },
        end: { x: 2, y: 1 },
      },
      {
        name: 'X',
        pos: { x: 1, y: 2 },
        end: { x: 1, y: 3 },
      },
      {
        name: 'O',
        pos: { x: 2, y: 2 },
        end: { x: 3, y: 3 },
      },
    ];

    return <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        width: '100%',
        height: '100%',
      }}
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          maxWidth: `${(newCharCount.width / areas.length) * emSize.width * areas.length}px`,
          maxHeight: `${(newCharCount.height / areas[0].length) * emSize.height * areas[0].length}px`,
          display: 'grid',
          gridTemplateRows: Array(areas.length).fill('1fr').join(' '),
          gridTemplateColumns: Array(areas[0].length).fill('1fr').join(' ')
        }}
      >
        {slots.map(slot => <div
          key={slot.name}
          style={{
            gridRow: `${slot.pos.y} / ${slot.end.y + 1}`,
            gridColumn: `${slot.pos.x} / ${slot.end.x + 1}`
          }}>
          <CharCountContext.Provider value={{
            width: Math.floor(newCharCount.width * ((slot.end.x - slot.pos.x) + 1) / areas[0].length),
            height: Math.floor(newCharCount.height * ((slot.end.y - slot.pos.y) + 1) / areas.length)
          }}>
            {(slot.content ?? (() => null))()}
          </CharCountContext.Provider>
        </div>)}
      </div>
    </div>;
  };

  const methods = {
    SetLayout(areas: string[][]) {
      state.areas = areas;
    }
  } as const;

  return FapWrap(component, methods);
}

type TUIGridCell = ReturnType<typeof TUIGridCell>;
export function TUIGridCell(content: FapContent) {
  return () => <div>{recursivePrepareContent(content)}</div>;
}