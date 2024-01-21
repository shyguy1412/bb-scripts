
import Style from '@/bb-plasma/style/Taskbar.css';
import { WindowManagerContext } from '@/bb-plasma/DesktopEnviroment';
import { PlasmaWindow } from '@/bb-plasma/components/PlasmaWindow';
import { useContext, type PropsWithChildren } from 'react';
import React from 'react';

type Props = Parameters<typeof PlasmaWindow>[0];

export function TaskbarEntry(props: PropsWithChildren<Props>) {
  const { title, minimized } = props;
  const [{ windows }, requestAction] = useContext(WindowManagerContext);
  const inFocus = windows.at(-1).id == props.id && !minimized;

  return <>
    <Style></Style>
    <div className={`taskbar-entry plasma-button plasma-box-right ${!inFocus || 'taskbar-entry-focus'}`}
      onClick={() => {
        if (minimized || !inFocus)
          requestAction({
            action: 'FOCUS',
            window: props
          });
        else
          requestAction({
            action: 'MINIMIZE',
            window: props
          });
      }}>
      {title}
    </div>
  </>;
}