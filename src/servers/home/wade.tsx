import React, { useContext, useState, useEffect } from "react";
import { getAllServers } from "@/lib/Network";
import { NetscriptContext } from "@/lib/Context";
import { createWindowApp } from "@/lib/WindowApp";

export async function main(ns: NS) {
  const WindowApp = createWindowApp(ns);
  ns.atExit(() => {
    WindowApp.cleanup();

  });
  return WindowApp.mount(<NetworkMap></NetworkMap>);
}

function NetworkMap() {
  const ns: NS = useContext(NetscriptContext);
  let [servers, setServers] = useState(getAllServers(ns).map(server => ns.getServer(server)));

  useEffect(() => {
    const interval = setInterval(() => {
      servers = getAllServers(ns).map(server => ns.getServer(server));
      setServers(servers);
      ns.print("Updating network");
    }, 500);

    return () => clearInterval(interval);
  }, []);

  return (
    <div id="networkMap">
      <table>
        <thead>
          <tr>
            <th>Server</th>
            <th>Parent</th>
            <th>RAM</th>
            <th>Rooted?</th>
            <th>Min. Hack</th>
          </tr>
        </thead>
        <tbody>
          {servers.map((server) =>
          (<tr key={server.hostname}>
            <td>{server.hostname}</td>
            <td>{server.parent !== "" ? server.parent : "N/A"}</td>
            <td>{ns.formatRam(server.maxRam)}</td>
            <td>{server.hasAdminRights ? "✓" : "❌"}</td>
            <td>{server.minDifficulty}</td></tr>)
          )}
        </tbody>
      </table>
    </div>
  );
}