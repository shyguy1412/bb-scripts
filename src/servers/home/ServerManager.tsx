import Style from '@/lib/ServerManager/ServerManager.css';
import { ServerEntry } from '@/lib/ServerManager/components/ServerEntry';
import { createWindowApp } from '@/lib/WindowApp';
import { List } from '@/lib/components/List';
import { getPurchasedServers } from '@/lib/util';
import { Server } from 'NetscriptDefinitions';
import React, { useRef, useState } from 'react';

export async function main(ns: NS) {
  const WindowApp = createWindowApp(ns);

  return WindowApp.mount(<WindowComponent ns={ns}></WindowComponent>);
}

type Props = {
  ns: NS;
};

function WindowComponent({ ns }: Props) {
  const [servers, setServers] = useState<Server[]>(getPurchasedServers(ns));
  const inputRef = useRef<HTMLInputElement>(null);
  const defaultNewServerName = `grindr-${+servers.at(-1).hostname.split('-')[1] + 1}`;
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
        {React.createElement(List,)}
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