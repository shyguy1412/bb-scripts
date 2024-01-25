import { findTailRoot, watchElForDeletion } from '@/lib/BitburnerDOM';
import React, { createContext } from 'react';
import ReactDOM from 'react-dom';

export const NetscriptContext = createContext<NS>(undefined);
export const CleanupContext = createContext(null);

export function createWindowApp(ns: NS) {
  const cleanupCallbacks: (() => void)[] = [];
  return {
    cleanup: () => {
      console.log('CLEANING');
      
      cleanupCallbacks.forEach(c => c());
      ns.tprint('Terminated');
      ns.closeTail();
    },
    async mount(component: React.JSX.Element) {
      ns.tail();
      ns.disableLog('ALL');
      ns.printRaw(<span data-pid={ns.pid}></span>);

      await ns.sleep(0); // give up control so DOM can update

      const root = findTailRoot(document.querySelector(`span[data-pid="${ns.pid}"]`));

      const WindowWrapper = () => {

        const theme = ns.ui.getTheme();
        Object.entries(theme).forEach(([key, value]) => {
          root.style.setProperty(`--${key}`, value);
        });

        root.style.flexDirection = 'unset';

        return <div style={{
          color: 'var(--primarylight)',
          width: '100%',
          height: '100%',
          fontFamily: '"Lucida Console", "Lucida Sans Unicode", "Fira Mono", Consolas, "Courier New", Courier, monospace, "Times New Roman"'
        }}>
          {component}
        </div>;
      };

      ReactDOM.render(<NetscriptContext.Provider value={ns}>
        <CleanupContext.Provider value={(f: () => void) => cleanupCallbacks.push(f)}>
          <WindowWrapper></WindowWrapper>
        </CleanupContext.Provider>
      </NetscriptContext.Provider>, root);

      return new Promise<void>(resolve => {
        watchElForDeletion(root, () => resolve());
      });
    }
  };
}

