import { useContext, useEffect, useState } from 'react';
import style from './ServerManager.css' with { type: 'css' };
import { NetscriptContext } from '@/lib/Context';
import { adoptStyle } from '@/lib/BitburnerDOM';
import React from 'react';
import { Server } from 'NetscriptDefinitions';
import { enable_hot_reload } from '@/lib/syscalls/hot_reload';

const SERVER_BASE_NAME = 'pserv';

export function ServerManager() {
    const ns = useContext(NetscriptContext);

    adoptStyle(ns, style);
    enable_hot_reload(ns);

    const servers = ns.cloud.getServerNames().map((s) => ns.getServer(s) as Server);

    const poll = useState(false)[1].bind(undefined, (p) => !p);

    useEffect(() => {
        const interval = setInterval(() => {
            poll();
        }, 500);

        return () => clearInterval(interval);
    }, [poll]);

    const entries = servers.map((s, i) =>
        <ServerEntry key={i} poll={poll} server={s}></ServerEntry>
    );

    const buyNewServer = (e: React.MouseEvent<HTMLButtonElement>) => {
        const input = e.currentTarget.parentElement?.querySelector('input')!;
        if (!ns.cloud.purchaseServer(input.value, 2)) {
            ns.toast('Failed to buy server', 'error');
        }
        poll();
    };

    return (
        <div className='server-manager'>
            <div>
                <span>
                    Purchase new server:
                    <input
                        type='text'
                        value={`${SERVER_BASE_NAME}-${servers.length}`}
                    />
                </span>
                <button onClick={buyNewServer}>OK</button>
            </div>
            <div className='server-manager-entries'>
                {entries}
            </div>
        </div>
    );
}

namespace ServerEntry {
    export type Props = {
        server: Server;
        poll: () => void;
    };
}

function ServerEntry({ server, poll }: ServerEntry.Props) {
    const ns = useContext(NetscriptContext);

    const doubleRam = () => doubleServerRam(ns, server.hostname) ? (poll(), true) : false;
    const maxoutRam = () => doubleRam() ? doubleRam() : (poll(), false);
    const deleteServer = () => (ns.cloud.deleteServer(server.hostname), poll());

    return <div>
        <div>{server.hostname} ({server.ip})</div>
        <div>RAM: {ns.format.ram(server.maxRam, 2)}</div>
        <div>
            <button onClick={doubleRam}>X2</button>
            <button onClick={maxoutRam}>MAX</button>
            <button onClick={deleteServer}>DELETE</button>
        </div>
    </div>;
}

function doubleServerRam(ns: NS, host: string) {
    return ns.cloud.upgradeServer(host, ns.getServerMaxRam(host) * 2);
}
