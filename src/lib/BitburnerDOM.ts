export function findTailRoot(span: HTMLElement) {
  let el = span;
  while (!el.parentElement.classList.contains('react-resizable'))
    el = el.parentElement;
  return el;
}

export function watchElForDeletion(elToWatch: Element, callback: () => void) {
  const observer = new MutationObserver(function (mutations) {

    // loop through all mutations
    mutations.forEach(function (mutation) {
      // check for changes to the child list
      if (mutation.type === 'childList') {
        mutation.removedNodes.forEach(node => {
          if (!containsRecursive(node, elToWatch)) return;
          callback();
          observer.disconnect();
        });
      }
    });
  });

  // start observing the dom
  observer.observe(document.body, { childList: true, subtree: true });

  return {
    cleanup: () => observer.disconnect()
  };
};

export function watchSelectorForCreation(selector: string, callback: (el: HTMLElement) => void) {
  const observer = new MutationObserver(function (mutations) {

    // loop through all mutations
    mutations.forEach(function (mutation) {
      const element = [...mutation.addedNodes].reduce((prev, cur) => {
        if (cur.nodeType != cur.ELEMENT_NODE) return prev;
        const added = (cur as HTMLElement).matches(selector) ? cur : (cur as HTMLElement).querySelector(selector);
        return prev ?? added;
      }, null) as HTMLElement;

      if (element) {
        callback(element);
        observer.disconnect();
      }
    });
  });

  // start observing the dom
  observer.observe(document.body, { childList: true, subtree: true });

  return {
    cleanup: () => observer.disconnect()
  };
};


export function containsRecursive(container: Node | Element, child: Element): boolean {
  if (!('children' in container)) return false;
  return [...container.children].reduce((prev, cur) => prev || cur == child || containsRecursive(cur, child), false);
}