import Style from './ServerManager.css';
import { ServerEntry } from './ServerEntry';
import { List } from '@/lib/components/List';
import { getPurchasedServers } from '@/lib/Network';
import { Server } from 'NetscriptDefinitions';
import React, { useState, useRef, useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';

export function ServerManager() {

  const ns = useContext(NetscriptContext);
  const [servers, setServers] = useState<Server[]>(getPurchasedServers(ns));
  const inputRef = useRef<HTMLInputElement>(null);
  const defaultNewServerName = `grindr-${+(servers.at(-1)?.hostname?.split('-')?.[1] ?? 0) + 1}`;

  return <>
    <Style></Style>
    <div className='server-manager-layout'>

      <div className='server-manager-purchase-server'>
        <span>
          Purchase new server:
          &nbsp;
          <input type='text' ref={inputRef} defaultValue={defaultNewServerName} key={defaultNewServerName} />
          &nbsp;
        </span>
        <span className='server-manager-purchase-server-button'
          onClick={() => {
            'use getPurchasedServerLimit';
            'use purchaseServer';

            if (!inputRef.current) throw new Error('Something went wrong, no input ref');
            const newServer = ns.purchaseServer(inputRef.current.value, 2);
            if (!newServer && servers.length == ns.getPurchasedServerLimit()) {
              ns.toast('Server limit has been reached', 'error');
              return;
            } else if (!newServer) {
              ns.toast('Not enough money', 'error');
              return;
            }
            ns.toast(`Successfully bought server '${newServer}'`, 'success');
            setServers(getPurchasedServers(ns));
          }}
        >OK</span>
      </div>
      <div className='server-manager'>
        <List li={ServerEntry} data={servers.map(s => ({
          ns,
          server: s,
          remove: () => {
            'use deleteServer';
            ns.deleteServer(s.hostname);
            setServers(getPurchasedServers(ns));
          }
        }))}></List>
      </div>
    </div>
  </>;
}