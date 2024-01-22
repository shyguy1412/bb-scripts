import Style from './ServerSection.css';
import { ServerEntry } from "@/Dolphin/ServerEntry";
import { List } from "@/lib/components/List";
import { Server } from "NetscriptDefinitions";
import React from "react";

type Props = {
  section?: string;
  servers: Server[];
};

export function ServerSection({ servers, section }: Props) {
  const list = <List data={servers.map(s => ({ server: s }))} li={ServerEntry}></List>;
  if (!section) return <div className='dolphin-server-section'>
    <Style></Style>
    {list}
  </div>;

  return <details className='dolphin-server-section'>
    <Style></Style>
    <summary>{section}</summary>
    <div className="dolphin-server-section-list">{list}</div>
  </details>;
};
