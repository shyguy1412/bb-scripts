import { NetscriptContext } from "@/lib/Context";
import { getAllServers } from "@/lib/Network";
import { DropDown } from "@/lib/components/DropDown";
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
    <DropDown value={server} options={servers} onChange={value => setServer(value)}></DropDown>
    <FontAwesomeIcon icon={faRotate} onClick={refresh}></FontAwesomeIcon>
    <pre>
      {JSON.stringify(ns.getServer(server), null, 2)}
    </pre>
  </div>;
}