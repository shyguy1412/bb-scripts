import Style from '@/bb-plasma/style/PlasmaWindow.css';
import { Draggable } from '@/bb-plasma/lib/Draggable';
import { faWindowMinimize, faXmark } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon, FontAwesomeIconProps } from '@fortawesome/react-fontawesome';
import { WindowManagerContext } from '@/bb-plasma/DesktopEnviroment';
import { Resizable } from '@/bb-plasma/lib/Resizable';
import { createContext, createRef, useContext, useEffect, useState, type PropsWithChildren } from 'react';
import React from 'react';

type Props = {
  title?: string;
  icon?: FontAwesomeIconProps['icon'];
  resizable?: boolean;
  minimized?: boolean;
  x?: number;
  y?: number;
  id: number;
};

let idCounter = 1;

export const PlasmaWindowContext = createContext<Props>(null);

export function createWindow(props: Omit<PropsWithChildren<Props>, 'id'>): Props {
  const id = idCounter++;
  return {
    ...props,
    id
  };
}

export function PlasmaWindow(props: PropsWithChildren<Props>) {
  const {
    title,
    children,
    resizable,
    minimized = false,
    x,
    y,
    icon
  } = props;

  const [{ windows }, requestAction] = useContext(WindowManagerContext);
  const [isDraggable, setDraggable] = useState(false);
  const inFocus = windows.at(-1).id == props.id && !minimized;
  const draggableRef = createRef<HTMLDivElement>();


  //effect to center the window if no start position was specified
  useEffect(() => {
    if (!draggableRef.current) return;
    draggableRef.current.style.left = (x ?? document.body.clientWidth / 2 - draggableRef.current.clientWidth / 2 + 'px') + '';
    draggableRef.current.style.top = (y ?? document.body.clientHeight / 2 - draggableRef.current.clientHeight / 2 + 'px') + '';
  }, []);

  return <>
    <Style></Style>
    <Draggable _ref={draggableRef} x={x} y={y} active={isDraggable}>
      <div style={{
        display: minimized ? 'none' : undefined
      }}
        className={`plasma-window ${inFocus ? 'plasma-box-glow' : ''} plasma-box`}
        onMouseDown={() => requestAction({
          action: 'FOCUS',
          window: props
        })}
      >
        <div
          onMouseDown={(e) => {
            setDraggable(true);
            addEventListener('mouseup', () => {
              setDraggable(false);
            }, { once: true });
          }} className='plasma-window-titlebar plasma-box-bottom'>
          <span>{!icon || <FontAwesomeIcon icon={icon}></FontAwesomeIcon>}{title}</span>
          <span>
            <div className='plasma-button plasma-fill plasma-square' onClick={() => requestAction({
              action: 'MINIMIZE',
              window: props
            })}>
              <FontAwesomeIcon style={{ 'fontSize': '0.9em' }} icon={faWindowMinimize}></FontAwesomeIcon>
            </div>
            <div className='plasma-button plasma-fill plasma-square' onClick={() => requestAction({
              action: 'CLOSE',
              window: props
            })}>
              <FontAwesomeIcon icon={faXmark}></FontAwesomeIcon>
            </div>
          </span>
        </div>
        <Resizable resizable={resizable}>
          <PlasmaWindowContext.Provider value={props}>
            <div className='plasma-window-content'>
              {children}
            </div>
          </PlasmaWindowContext.Provider>
        </Resizable>
      </div>
    </Draggable>
  </>;
}