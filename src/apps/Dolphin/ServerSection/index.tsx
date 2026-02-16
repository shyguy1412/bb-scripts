import style from './ServerSection.css' with {type: 'css'};
import { ServerEntry } from "../ServerEntry";
import { List } from "@/lib/components/List";
import { Server } from "NetscriptDefinitions";
import React, { useContext } from "react";
import { adoptStyle } from '@/lib/BitburnerDOM';
import { NetscriptContext } from '@/lib/Context';

type Props = {
    section?: string;
    servers: Server[];
};

export function ServerSection({ servers, section }: Props) {
    const ns = useContext(NetscriptContext);
    adoptStyle(ns, style);

    const list = <List data={servers.map(s => ({ server: s }))} li={ServerEntry}></List>;
    if (!section) return <div className='dolphin-server-section'>
        {list}
    </div>;

    return <details className='dolphin-server-section'>
        <summary>{section}</summary>
        <div className="dolphin-server-section-list">{list}</div>
    </details>;
};
