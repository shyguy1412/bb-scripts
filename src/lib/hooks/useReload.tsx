import { NetscriptContext, TerminateContext, CleanupContext, TailRootContext } from "@/lib/Context";
import { FapComponents } from "@/lib/FapUI";
import { faRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import { useContext, useEffect } from "react";
import ReactDOM from "react-dom";

export function useReload() {

  const ns = useContext(NetscriptContext);
  const terminate = useContext(TerminateContext);
  const addCleanup = useContext(CleanupContext);
  const root = useContext(TailRootContext);

  const getTitlebarElement = (action: HTMLElement | null) => {
    let el = action;
    while (!el?.matches('.react-resizable') && el?.parentElement) el = el.parentElement;
    el = (el?.firstElementChild?.lastElementChild ?? null) as HTMLElement | null;
    return el;
  };

  const titlebarElement = getTitlebarElement(root);

  useEffect(() => {
    if (!titlebarElement) return;

    const { Fragment, RawElement } = FapComponents;
    const [, ...children] = [...titlebarElement.children];

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
    </>, titlebarElement);

    titlebarElement.style.display = 'flex';

  }, [titlebarElement]);
}