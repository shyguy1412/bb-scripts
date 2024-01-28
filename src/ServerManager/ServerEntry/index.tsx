import { doublePurchasedServerRam, maxoutPurchasedServerRam } from '@/lib/Network';
import Style from './ServerEntry.css';
import { Server } from 'NetscriptDefinitions';
import React, { useState } from 'react';

type Props = {
  server: Server;
  remove: () => void;
  ns: NS;
};

export function ServerEntry({ server: { hostname }, ns, remove }: Props) {

  'use getServer';
  const [server, setServer] = useState<Server>(ns.getServer(hostname));

  return <>
    <Style></Style>
    <div className='server-entry'>
      <div>
        {server.hostname} ({server.ip})
      </div>
      <div>
        <span>RAM: {ns.formatRam(server.maxRam)}</span>
      </div>
      <div className='server-entry-button-wrapper'>
        <span className='server-entry-button'
          onClick={() => {
            'use getServerMaxRam';

            if (doublePurchasedServerRam(ns, server.hostname))
              ns.toast('Server upgraded successfully', 'success');
            else if (server.maxRam == ns.getServerMaxRam(hostname))
              ns.toast('Server is already maxed out', 'info');
            else
              ns.toast('Not enough money', 'error');

            setServer(ns.getServer(hostname));
          }}
        >x2</span>
        <span className='server-entry-button'
          onClick={() => {
            maxoutPurchasedServerRam(ns, server.hostname);
            if (ns.getPurchasedServerMaxRam() == ns.getServerMaxRam(hostname))
              ns.toast('Server is already maxed out', 'info');
            else
              ns.toast('Server upgraded to max affordable RAM', 'success');
            setServer(ns.getServer(hostname));
          }}
        >max</span>
        <span className='server-entry-button'
          onClick={() => remove()}
        >DELETE</span>
      </div>
    </div>
  </>;
}