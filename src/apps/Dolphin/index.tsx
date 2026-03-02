import { FileGrid } from '@/lib/components/FileGrid';
import style from './Dolphin.css' with {type: "css"};
import { ServerSection } from './ServerSection';
import { List } from '@/lib/components/List';
import { get_all_servers } from '@/lib/Network';
import React, { Dispatch, SetStateAction, createContext, useContext, useEffect, useMemo, useState } from 'react';
import { BreadCrumbs } from './BreadCrumbs';
import { list_directory } from '@/lib/FileSystem';
import { NetscriptContext } from '@/lib/Context';
// import { getAllCodingContracts } from '@/lib/FileSystem';
// import { useReload } from '@/lib/hooks/useReload';
import { FaPlus } from 'react-icons/fa';
import { adoptStyle } from '@/lib/BitburnerDOM';
import { enable_hot_reload } from '@/lib/syscalls/hot_reload';
import { connect_to_fdaemon } from '@/home/bin/service/fdaemon';
import { alive } from '@/lib/System';

// @ts-expect-error
export const PathContext = createContext<([string, Dispatch<SetStateAction<string>>])>(null);

export function Dolphin() {
    'use exec';
    'use getHostname';
    'use getServer';

    const ns = useContext(NetscriptContext);

    enable_hot_reload(ns);
    adoptStyle(ns, style);

    const [path, setPath] = useState<string>(ns.args[0] as string ?? ns.getHostname());

    const servers = get_all_servers(ns);
    const sections = servers.reduce((prev, cur) => {
        const server = ns.getServer(cur);

        if (server.hostname == 'home') {
            prev.unshift({ servers: [server] });
            return prev;
        }

        if (server.purchasedByPlayer) {
            const label = 'purchased';
            const section = prev.find(s => s.section == label);
            if (section) section.servers.push(server);
            else prev.push({
                section: label,
                servers: [server],
            });
            return prev;
        }

        if (!server.purchasedByPlayer) {
            const label = 'other';
            const section = prev.find(s => s.section == label);
            if (section) section.servers.push(server);
            else prev.push({
                section: label,
                servers: [server],
            });
            return prev;
        }

        return prev;
    }, [] as Parameters<typeof ServerSection>[0][]);

    ns.ui.setTailTitle(`Dolphin - ${path.replace(/([^\/]*)(\/?)/, '$1://')}`);
    const [files, setFiles] = useState(() => list_directory(ns, path.split('/')[1] ?? '', { withFileTypes: true }));

    useEffect(() => {
        if (!alive(ns)) return;
        const [write, read] = connect_to_fdaemon(ns);
        write("subscribe", {
            event: 'change',
            path: path
        });

        read().finally(() => setFiles(list_directory(ns, path, { withFileTypes: true })));
    }, [files]);


    if (files)
        return <>
            <PathContext.Provider value={[path, setPath]}>
                <div className='dolphin-layout'>
                    <div className='dolphin-actionbar'>
                        <BreadCrumbs></BreadCrumbs>
                        <span className='dolphin-actions'>
                            <span
                                onClick={() => {
                                    // mkdir(ns, `${path}/new_dir`);
                                }}
                            >
                                <FaPlus style={{ marginRight: '0.2em' }}></FaPlus>
                                new folder
                            </span>
                            <span
                                onClick={() => {
                                    // writeFile(ns, '', `${path}/new_file.js`);
                                }}
                            >
                                <FaPlus style={{ marginRight: '0.2em' }}></FaPlus>
                                new file
                            </span>
                        </span>
                    </div>
                    <div className='dolphin-explorer'>
                        <List data={sections.map(s => ({ ...s }))} li={ServerSection} ></List>
                        <div
                            className='dolphin-explorer-button'
                            onClick={() => (setPath('~home/coding-contracts'))}
                        >Coding Contracs</div>
                    </div>
                    <div className='dolphin-content'>
                        {/* <DoubleClickFileContext.Provider value={(e, { type, name }) => {
                            switch (type) {
                                case 'js':
                                    const [server, script] = `${path}/${name}`.split(/\/(.*)/, 2);
                                    ns.exec(script, server);
                                    break;
                                case 'folder':
                                    setPath(`${path}/${name}`);
                                    break;
                                case 'txt':
                                    // ns.alert(readFile(ns, `${path}/${name}`));
                                    break;
                                case 'exe':
                                    ns.toast('.exe files can only be run from the terminal', 'error');
                                    break;
                                default:
                                    ns.toast(`This filetype is not supported (${type})`, 'error');
                                    break;
                            }
                        }}
                        >
                        </DoubleClickFileContext.Provider> */}
                        <FileGrid files={files} path={path}></FileGrid>
                    </div>
                </div>
            </PathContext.Provider>
        </>;

    ns.toast("Current folder was deleted, moving to home", "warning");

    return setPath('home') as undefined;
}