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

export function watchSelectorForCreation(selector: string, callback: () => void) {
  const observer = new MutationObserver(function (mutations) {

    // loop through all mutations
    mutations.forEach(function (mutation) {
      const selectorWasAdded = [...mutation.addedNodes].some(node =>
        node.nodeType == node.ELEMENT_NODE && (node as HTMLElement).querySelector(selector) != null
      );

      if (selectorWasAdded) {
        callback();
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


export function containsRecursive(container: Node | Element, child: Element) {
  if (!('children' in container)) return;
  return [...container.children].reduce((prev, cur) => prev || cur == child || containsRecursive(cur, child), false);
}