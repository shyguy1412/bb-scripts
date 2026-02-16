import { findTailRoot, watchElForDeletion } from '@/lib/BitburnerDOM';
import { NetscriptContext, TerminateContext, TailRootContext } from '@/lib/Context';
import React from 'react';
import { createPortal } from 'react-dom';

export async function createWindowApp(ns: NS, Component: React.FunctionComponent) {
  ns.ui.openTail();
  ns.disableLog('ALL');
  ns.printRaw(<span data-pid={ns.pid}></span>);
  await ns.sleep(0); // give up control so DOM can update
  const root = findTailRoot(document.querySelector(`span[data-pid="${ns.pid}"]`)!);

  const controller = new AbortController();
  ns.atExit(() => controller.abort(), crypto.randomUUID());

  const theme = ns.ui.getTheme();
  Object.entries(theme).forEach(([key, value]) => {
    root.parentElement!.parentElement!.style.setProperty(`--${key}`, value!);
  });

  root.style.flexDirection = 'unset';

  return new Promise<void>(async resolve => {
    watchElForDeletion(root, () => resolve(), controller.signal);

    ns.printRaw(<>
      {createPortal(
        <NetscriptContext.Provider value={ns}>
          <TerminateContext.Provider value={resolve}>
            <TailRootContext.Provider value={root}>
              <div style={{
                position: 'relative',
                color: 'var(--bb-theme-primarylight)',
                width: '100%',
                height: '100%',
                fontFamily: '"Lucida Console", Consolas, Courier, monospace'
              }}>
                <Component></Component>
              </div>
            </TailRootContext.Provider>
          </TerminateContext.Provider>
        </NetscriptContext.Provider>
        , root
      )}
    </>);
  });
}

export const mainWrapper = (Component: React.FunctionComponent) =>
  (ns: NS) => createWindowApp(ns, Component).catch(console.error)
  ;

export { NetscriptContext, TerminateContext } from '@/lib/Context';
