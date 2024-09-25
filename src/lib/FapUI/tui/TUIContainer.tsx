import { FapComponent, FapContent, FapElement, recursivePrepareContent } from "@/lib/FapUI";
import { sleep } from "@/lib/System";
import React, { Children, createContext, PropsWithChildren, RefCallback, useContext, useEffect, useRef, useState } from "react";


//@ts-expect-error
export const EmContext = createContext<{ width: number, height: number; }>(null);
//@ts-expect-error
export const CharCountContext = createContext<{ width: number, height: number; }>(null);

type TUIContainerProps = {
} & PropsWithChildren;

export function TUI(content?: FapContent) {
  return React.createElement(TUIContainer, {}, recursivePrepareContent(content));
}

export function TUIContainer({ children }: TUIContainerProps) {

  const [ref, em] = useWidthReference();

  return <div
    style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      width: '100%',
      height: '100%',
    }}
  >
    <div ref={ref} style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      flexDirection: 'column',
      fontFamily: '"Lucida Console", "Lucida Sans Unicode", "Fira Mono", Consolas, "Courier New", Courier, monospace, "Times New Roman"',
      width: '100%',
      height: '100%',
    }}>
      <EmContext.Provider value={em}>
        <AsciiContainer>{children}</AsciiContainer>
      </EmContext.Provider>
    </div >
  </div>;
}

function useWidthReference() {
  const [reference, setReference] = useState<HTMLElement>();
  const [width, setWidth] = useState(0);
  const [height, setHeight] = useState(0);

  useEffect(() => {
    if (!reference) return;
    reference.style.opacity = '0';
    reference.style.width = 'min-content';
    reference.style.position = 'absolute';
    reference.textContent = 'm';

    const onResize = async () => {
      setWidth(Math.round(reference!.getBoundingClientRect().width * 100) / 100);
      setHeight(Math.round(reference!.getBoundingClientRect().height * 100) / 100);
    };

    const observer = new ResizeObserver(onResize);

    observer.observe(reference);

    onResize();

    return () => observer.disconnect();

  }, [reference]);

  return [
    (el: HTMLElement | null) => el && !reference && setReference((el.append(document.createElement('div')), el.lastElementChild as HTMLElement)),
    { width, height }
  ] as const;
}

type AsciiContainerProps = {
} & PropsWithChildren;

function AsciiContainer({ children }: AsciiContainerProps) {

  const ref = useRef<HTMLDivElement>(null);
  const [charWidth, setCharWidth] = useState(0);
  const [charHeight, setCharHeight] = useState(0);
  const { width: emWidth, height: emHeight } = useContext(EmContext);

  useEffect(() => {
    if (!ref.current || !emWidth) return;

    const parent = ref.current.parentElement!;

    const observer = new ResizeObserver(() => {
      setCharWidth(Math.floor(parent.clientWidth / emWidth));
      setCharHeight(Math.floor(parent.clientHeight / emHeight));
    });

    observer.observe(parent);

    setCharWidth(Math.floor(parent.clientWidth / emWidth));
    setCharHeight(Math.floor(parent.clientHeight / emHeight));

    return () => observer.disconnect();
  }, [ref, emWidth]);

  return <div ref={ref} style={{
    width: `${charWidth * emWidth}px`,
    height: `${charHeight * emHeight}px`,
  }}>
    <CharCountContext.Provider value={{ width: charWidth, height: charHeight }}>
      {children}
    </CharCountContext.Provider>
  </div>;
}