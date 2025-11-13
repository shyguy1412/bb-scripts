import style from './ServerSection.css' with {type: 'css'};
import { ServerEntry } from "../ServerEntry";
import { List } from "@/lib/components/List";
import { Server } from "NetscriptDefinitions";
import React from "react";
import { adoptStyle } from '@/lib/hooks/useStyle';

type Props = {
  section?: string;
  servers: Server[];
};

export function ServerSection({ servers, section }: Props) {
  adoptStyle(style);

  const list = <List data={servers.map(s => ({ server: s }))} li={ServerEntry}></List>;
  if (!section) return <div className='dolphin-server-section'>
    {list}
  </div>;

  return <details className='dolphin-server-section'>
    <summary>{section}</summary>
    <div className="dolphin-server-section-list">{list}</div>
  </details>;
};
