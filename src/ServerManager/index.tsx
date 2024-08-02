import Style from './ServerManager.css';
import { ServerEntry } from './ServerEntry';
import { List } from '@/lib/components/List';
import { doublePurchasedServerRam, getAllServers, getPurchasedServers } from '@/lib/Network';
import { Server } from 'NetscriptDefinitions';
import React, { useState, useRef, useContext } from 'react';
import { NetscriptContext } from '@/lib/Context';

export function ServerManager() {
  'use getPurchasedServerLimit';
  'use getServer';
  'use purchaseServer';
  'use deleteServer';

  const ns = useContext(NetscriptContext);
  const [servers, setServers] = useState<Server[]>(getPurchasedServers(ns));
  const inputRef = useRef<HTMLInputElement>(null);
  const defaultNewServerName = `${ns.args[0]??'grindr'}-${+(servers.at(-1)?.hostname?.split('-')?.[1] ?? 0) + 1}`;

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
        <span className='server-manager-button-list'>
          <span className='server-manager-purchase-server-button'
            onClick={() => {
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
          <span className='server-manager-auto-upgrade-button'
            onClick={() => {

              const servers = getPurchasedServers(ns);
              servers.sort((a, b) => a.maxRam - b.maxRam);

              while (doublePurchasedServerRam(ns, servers[0].hostname)) {
                servers[0].maxRam *= 2;
                servers.sort((a, b) => a.maxRam - b.maxRam);
              };
              ns.toast(`Upgraded all servers`, 'success');
              setServers(getPurchasedServers(ns));
            }}
          >AUTO</span>
        </span>
      </div>
      <div className='server-manager'>
        <List li={ServerEntry} data={servers.map(s => ({
          ns,
          server: s,
          remove: () => {
            ns.deleteServer(s.hostname);
            setServers(getPurchasedServers(ns));
          }
        }))}></List>
      </div>
    </div>
  </>;
}