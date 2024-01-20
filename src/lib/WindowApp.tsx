import React from "react";
import ReactDOM from "react-dom";

export function createWindowApp(ns: NS) {
  return {
    async mount(component: React.JSX.Element) {
      ns.tail();
      ns.disableLog('ALL');
      ns.printRaw(<span data-pid={ns.pid}></span>);

      await ns.sleep(0); // give up control so DOM can update

      const root = findRoot(document.querySelector(`span[data-pid="${ns.pid}"]`));

      const WindowWrapper = () => {

        const theme = ns.ui.getTheme();
        Object.entries(theme).forEach(([key, value]) => {
          root.style.setProperty(`--${key}`, value);
        });

        return <div style={{
          color: 'var(--primarylight)',
          fontFamily: '"Lucida Console", "Lucida Sans Unicode", "Fira Mono", Consolas, "Courier New", Courier, monospace, "Times New Roman"'
        }}>
          {component}
        </div>;
      };

      ReactDOM.render(<WindowWrapper></WindowWrapper>, root);

      ns.atExit(() => {
        ns.tprint('Terminated');
        ns.closeTail();
      });

      ns.sleep(500).then(() => {
        ns.print(8);
        ns.print(7);
        ns.print(6);
        ns.print(5);
        ns.print(4);
        ns.print(3);
        ns.print(2);
        ns.print(1);
      });

      return new Promise<void>(resolve => {
        watchElForDeletion(root, () => resolve());
      });
    }
  };
}

function findRoot(span: HTMLElement) {
  let el = span;
  while (!el.parentElement.classList.contains('react-resizable'))
    el = el.parentElement;
  return el;
}

function watchElForDeletion(elToWatch: Element, callback: () => void) {
  const parent = document.body;
  const observer = new MutationObserver(function (mutations) {

    // loop through all mutations
    mutations.forEach(function (mutation) {
      // check for changes to the child list
      if (mutation.type === 'childList') {
        mutation.removedNodes.forEach(node => !containsRecursive(node, elToWatch) || callback());
      }
    });
  });
  // start observing the parent - defaults to document body
  observer.observe(parent, { childList: true, subtree: true });
};

function containsRecursive(container: Node | Element, child: Element) {
  if (!('children' in container)) return;
  return [...container.children].reduce((prev, cur) => prev || cur == child || containsRecursive(cur, child), false);
}