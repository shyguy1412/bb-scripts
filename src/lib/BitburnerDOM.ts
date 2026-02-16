export function findTailRoot(span: HTMLElement) {
    let el = span;
    while (!el.parentElement!.classList.contains('react-resizable'))
        el = el.parentElement!;
    return el;
}

export function watchElForDeletion(elToWatch: Element, callback: () => void, signal?: AbortSignal) {
    const observer = new MutationObserver(function () {
        if (document.body.contains(elToWatch)) return;
        callback();
        observer.disconnect();
    });

    signal?.addEventListener("abort", () => observer.disconnect());

    observer.observe(document.body, { childList: true, subtree: true });
};

export function adoptStyle(ns: NS, style: CSSStyleSheet) {
    if (document.adoptedStyleSheets.includes(style)) return;

    document.adoptedStyleSheets.push(style);

    ns.atExit(
        () => {
            const index = document.adoptedStyleSheets.indexOf(style);
            document.adoptedStyleSheets.splice(index, 1);
        }
        , crypto.randomUUID()
    );

}