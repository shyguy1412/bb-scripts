import { findTailRoot, watchElForDeletion } from '@/lib/BitburnerDOM';
import { NetscriptContext, CleanupContext, TerminateContext, ContextCollection } from '@/lib/Context';
import React from 'react';
import ReactDOM from 'react-dom';

export function createWindowApp(ns: NS, pid?: string | number) {
  const cleanupCallbacks: (() => void)[] = [];
  return {
    cleanup: () => {
      cleanupCallbacks.forEach(c => c());
      ns.tprint('Terminated');
      ns.closeTail();
    },
    async mount(component: React.JSX.Element) {
      return new Promise<void>(async resolve => {
        ns.tail();
        ns.disableLog('ALL');
        ns.printRaw(<span data-pid={ns.pid}></span>);

        await ns.sleep(0); // give up control so DOM can update

        const root = findTailRoot(document.querySelector(`span[data-pid="${pid ?? ns.pid}"]`)!);

        const WindowWrapper = () => {

          const theme = ns.ui.getTheme();
          Object.entries(theme).forEach(([key, value]) => {
            root.style.setProperty(`--${key}`, value!);
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

        const contexts = [
          {
            context: NetscriptContext,
            value: ns
          },
          {
            context: TerminateContext,
            value: resolve
          },
          {
            context: CleanupContext,
            value: (f: () => void) => cleanupCallbacks.push(f)
          }
        ];

        cleanupCallbacks.push(() => ReactDOM.unmountComponentAtNode(root)); //this ensures the app is properly dismounted before NS is invalid

        ReactDOM.render(
          <ContextCollection contexts={contexts}>
            <WindowWrapper></WindowWrapper>
          </ContextCollection>
          , root);

        watchElForDeletion(root, () => resolve());
      });
    }
  };
}

export { NetscriptContext, CleanupContext, TerminateContext } from '@/lib/Context';
