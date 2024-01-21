import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useEffect, useRef, useState, type PropsWithChildren } from 'react';

type Props = {
  maxWidth?: number;
  maxHeight?: number;
  resizable?: boolean;
};

export function Resizable({ resizable = true, children }: PropsWithChildren<Props>) {

  const [size, setSize] = useState({ w: 0, h: 0 });
  const [minSize, setMinSize] = useState({ w: undefined, h: undefined } as typeof size);
  const ref = useRef<HTMLDivElement>();

  function mouseMove(e: MouseEvent) {
    setSize({
      w: (e.clientX) - ref.current.getBoundingClientRect().x,
      h: (e.clientY) - ref.current.getBoundingClientRect().y,
    });
  }

  useEffect(() => {
    if (!ref.current) return;
    setMinSize({
      w: ref.current.clientWidth,
      h: ref.current.clientHeight
    });
  }, [ref]);

  return <div
    ref={ref}
    style={{
      position: 'relative',
      width: Math.max(minSize.w, size.w) || 'inherit',
      height: Math.max(minSize.h, size.h) || 'inherit',
    }}
  >
    {children}
    {!resizable || <div
      style={{
        position: 'absolute',
        right: '-3px',
        bottom: '-3px',
        display: 'flex',
        cursor: 'nwse-resize',
      }}
      onMouseDown={(e) => {
        if (!resizable) return;
        if (!ref.current) return;
        const listener = (e: MouseEvent) => mouseMove(e);
        addEventListener('mousemove', listener);
        addEventListener('mouseup', () => {
          removeEventListener('mousemove', listener);
        }, { once: true });
      }}
    >
      <FontAwesomeIcon style={{
        transform: 'rotate(-45deg)',
        'aspectRatio': '1/1'
      }} icon={faCaretDown}></FontAwesomeIcon>
    </div>}
  </div >;
}