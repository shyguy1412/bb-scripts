import { findTailRoot, watchElForDeletion } from '@/lib/BitburnerDOM';
import { NetscriptContext, CleanupContext, TerminateContext, TailRootContext } from '@/lib/Context';
import React from 'react';
import ReactDOM from 'react-dom';

export function createWindowApp(ns: NS, pid?: string | number) {
  const cleanupCallbacks: (() => void)[] = [];
  return {
    cleanup: () => {
      cleanupCallbacks.forEach(c => c());
      // ns.tprint('Terminated');
      ns.ui.closeTail();
    },
    async mount(component: React.JSX.Element) {
      return new Promise<void>(async resolve => {
        ns.ui.openTail();
        ns.disableLog('ALL');
        ns.printRaw(<span data-pid={ns.pid}></span>);

        await ns.sleep(0); // give up control so DOM can update
        const root = findTailRoot(document.querySelector(`span[data-pid="${pid ?? ns.pid}"]`)!);

        const WindowWrapper = () => {

          const theme = ns.ui.getTheme();
          Object.entries(theme).forEach(([key, value]) => {
            root.parentElement!.parentElement!.style.setProperty(`--${key}`, value!);
          });

          root.style.flexDirection = 'unset';

          return <div style={{
            position: 'relative',
            color: 'var(--primarylight)',
            width: '100%',
            height: '100%',
            fontFamily: '"Lucida Console", "Lucida Sans Unicode", "Fira Mono", Consolas, "Courier New", Courier, monospace, "Times New Roman"'
          }}>
            {component}
          </div>;
        };

        const addCleanup = (f: () => void) => cleanupCallbacks.push(f);

        cleanupCallbacks.push(() => ReactDOM.unmountComponentAtNode(root)); //this ensures the app is properly dismounted before NS is invalid

        ReactDOM.render(
          <NetscriptContext.Provider value={ns}>
            <TerminateContext.Provider value={resolve}>
              <CleanupContext.Provider value={addCleanup}>
                <TailRootContext.Provider value={root}>
                  <WindowWrapper></WindowWrapper>
                </TailRootContext.Provider>
              </CleanupContext.Provider>
            </TerminateContext.Provider>
          </NetscriptContext.Provider>
          , root);

        watchElForDeletion(root, () => resolve());
      });
    }
  };
}

export const mainWrapper = (Component: React.FunctionComponent) => {
  return (ns: NS) => {
    try {
      const windowApp = createWindowApp(ns);

      ns.atExit(() => windowApp.cleanup());

      return windowApp.mount(<Component></Component>);
    } catch (error) {
      console.error(error);
    }
  };
};

export { NetscriptContext, CleanupContext, TerminateContext } from '@/lib/Context';
