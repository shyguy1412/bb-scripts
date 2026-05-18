import { getRequire } from '@/lib/exploit/WebpackRequire';
import { enable_hot_reload } from '@/lib/syscalls/hot_reload';
import { mainWrapper } from '@/lib/WindowApp';
import React, { Dispatch, useState } from 'react';

export async function main(ns: NS) {
    const require = getRequire();
    const Editor = require(146070)['J'];
    ns.ui.openTail();

    // enable_hot_reload(ns);

    let setState: Dispatch<Map<string, string>>;

    mainWrapper(
        () => {
            const [files, setFiles] = useState(new Map());
            setState = setFiles;
            return <Editor
                {...{
                    files,
                    hostname: 'home',
                    vim: false,
                }}
            />;
        },
    )(ns);

    setTimeout(
        () => setState(new Map([['home/editor.js', ns.read('home/editor.js')]])),
        1000,
    );

    return new Promise(() => {});
}
