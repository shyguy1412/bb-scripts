import { CleanupContext, NetscriptContext, TerminateContext } from "@/lib/Context";
import { FapComponents } from "@/lib/FapUI";
import { faRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useContext, useEffect, useReducer } from "react";
import ReactDOM from "react-dom";

function useReload() {

  const ns = useContext(NetscriptContext);
  const terminate = useContext(TerminateContext);
  const addCleanup = useContext(CleanupContext);

  const [titelbarElement, ref] = useReducer((prevState: HTMLElement | null, action: HTMLElement | null) => {
    let el = action;
    while (!el?.matches('.react-resizable') && el?.parentElement) el = el.parentElement;
    el = (el?.firstElementChild?.lastElementChild ?? null) as HTMLElement | null;
    return el;
  }, null);


  useEffect(() => {
    if (!titelbarElement) return;

    const { Fragment, RawElement } = FapComponents;
    const [stop, ...children] = [...titelbarElement.children];

    const ReloadButton = () => {
      return <button
        onClick={() => (terminate(), addCleanup(() => ns.run(ns.getScriptName())))}
        style={{
          background: 'var(--background)',
          border: 'none',
          borderLeft: '2px solid var(--welllight)',
          height: '100%',
          margin: 0,
          padding: 0,
          boxSizing: 'border-box',
          display: 'inline-flex',
          justifyContent: 'center',
          alignItems: 'center',
          fontSize: '12px',
          width: '2em'
        }}
      >
        <FontAwesomeIcon style={{ color: 'var(--primary)', fontSize: '17px' }} icon={faRotate}></FontAwesomeIcon>
      </button>;
    };

    ReactDOM.render(<>
      {
        Fragment([
          // RawElement(stop),
          ReloadButton,
          ...children.map(RawElement),
        ])
      }
    </>, titelbarElement);

    titelbarElement.style.display = 'flex';

  }, [titelbarElement]);

  return ref;
}

export function GraphBurner() {

  const ref = useReload();

  return <div
    ref={ref}
    style={{ width: '100%', height: '100%', color: '--primary' }}>
  </div>;
}