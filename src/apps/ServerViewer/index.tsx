import { NetscriptContext } from "@/lib/Context";
import { getAllServers } from "@/lib/Network";
import { faRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useContext, useReducer, useState } from "react";

export function ServerViewer() {
  "use getServer";
  const ns = useContext(NetscriptContext);

  const servers = getAllServers(ns);
  const [server, setServer] = useState(servers[0]);

  const [, refresh] = useReducer((state: boolean) => {
    return !state;
  }, true);


  return <div>
    <select
      style={{
        color: 'var(--primary)',
        backgroundColor: 'var(--backgroundprimary)',
        outline: 'none',
        fontFamily: 'inherit'
      }}
      value={server}
      onChange={({ currentTarget: { value } }) => setServer(value)}
    >
      {
        servers.map(server => <option style={{ background: 'var(--background)' }} key={server} value={server}>{server}</option>)
      }
    </select>

    <FontAwesomeIcon icon={faRotate} onClick={refresh}></FontAwesomeIcon>
    <pre>
      {JSON.stringify(ns.getServer(server), null, 2)}
    </pre>
  </div>;
}