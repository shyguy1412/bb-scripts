import { FapComponents, FapContent, FapElement, FapWrap, recursivePrepareContent, useFap } from "@/lib/FapUI";
import { CharCountContext, EmContext } from "@/lib/FapUI/tui/TUIContainer";
import React, { useMemo } from "react";
import { useContext } from "react";

export function TUIGrid(cells: FapContent[]) {

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

    const slots = useMemo(() => {
      const slots: Record<string, {
        name: string,
        pos: { x: number, y: number; };
        end: { x: number, y: number; };
        content: FapContent | null;
      }> = {};
      let cellIndex = 0;
      for (let y = 1; y <= areas.length; y++) {
        for (let x = 1; x <= areas[y - 1].length; x++) {
          const name = areas[y - 1][x - 1];
          if (name == '.') continue;
          if (!slots[name]) {
            slots[name] = {
              name,
              pos: { x, y },
              end: { x, y },
              content: cells[cellIndex++]
            };
          }
          slots[name].end.x = Math.max(slots[name].end.x, x);
          slots[name].end.y = Math.max(slots[name].end.y, y);
        }
      }
      return Object.values(slots);
    }, [areas]);


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
          gridTemplateRows: Array(areas.length).fill('minmax(0, 1fr)').join(' '),
          gridTemplateColumns: Array(areas[0].length).fill('minmax(0, 1fr)').join(' '),
          background: 'red',
          color: 'black'
        }}
      >
        {slots.map(slot => <div
          key={slot.name}
          style={{
            gridRow: `${slot.pos.y} / ${slot.end.y + 1}`,
            gridColumn: `${slot.pos.x} / ${slot.end.x + 1}`
          }}>
          <div
            style={{
              width: '100%',
              height: '100%',
              overflow: 'hidden'
            }}
          >
            <CharCountContext.Provider value={{
              width: Math.floor(newCharCount.width * ((slot.end.x - slot.pos.x) + 1) / areas[0].length),
              height: Math.floor(newCharCount.height * ((slot.end.y - slot.pos.y) + 1) / areas.length)
            }}>

              {recursivePrepareContent(slot.content)}
            </CharCountContext.Provider>
          </div>
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