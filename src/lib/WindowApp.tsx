import React, { createContext } from 'react';

export function adoptStyle(ns: NS, css: string) {
    const style = new CSSStyleSheet();
    style.replaceSync(css);

    globalThis['document'].adoptedStyleSheets.push(style);

    ns.atExit(
        () => {
            const index = globalThis['document'].adoptedStyleSheets.indexOf(style);
            globalThis['document'].adoptedStyleSheets.splice(index, 1);
        },
        crypto.randomUUID(),
    );
}

//@ts-expect-error its annoying to do constant null checks
export const NetscriptContext = createContext<NS>(null);

export async function createWindowApp(ns: NS, Component: React.FunctionComponent) {
    ns.ui.openTail();
    ns.disableLog('ALL');
    ns.clearLog();
    ns.atExit(() => ns.clearLog(), '__clear_log');
    ns.printRaw(
        <NetscriptContext.Provider value={ns}>
            <Component></Component>
        </NetscriptContext.Provider>,
    );
    ns.ui.renderTail();

    const cssPath = ns.self().filename.replace(/\.js$/, '.css');

    adoptStyle(ns, ns.read(cssPath));

    return new Promise(() => {});
}

export const mainWrapper = (Component: React.FunctionComponent) => (ns: NS) =>
    createWindowApp(ns, Component).catch(console.error);
