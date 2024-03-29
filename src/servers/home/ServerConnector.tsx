import { NetscriptContext } from "@/lib/Context";
import { getAllServers, getConnectionPath } from "@/lib/Network";
import { Terminal } from "@/lib/Terminal";
import { createWindowApp } from "@/lib/WindowApp";
import { List } from "@/lib/components/List";
import { faDoorOpen } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import { useContext } from "react";

export async function main(ns: NS) {
  const WindowApp = createWindowApp(ns);

  ns.atExit(() => WindowApp.cleanup());

  return WindowApp.mount(
    <App></App>
  );
}

function App() {
  const ns = useContext(NetscriptContext);

  const servers = getAllServers(ns);

  return <List data={servers.map(server => ({ server }))} li={({ server }: { server: string; }) =>
    <div
      onClick={() => {
        const term = new Terminal(ns);
        term.exec(getConnectionPath(ns, server).reduce((prev, cur) => prev + `connect ${cur};`, ''));
        term.cleanup();
      }}
    >{server}
      <span>
        <FontAwesomeIcon icon={faDoorOpen} onClick={() => {
          const term = new Terminal(ns);
          term.exec(getConnectionPath(ns, server).reduce((prev, cur) => prev + `connect ${cur};`, '') + ';backdoor');
          term.cleanup();
        }}></FontAwesomeIcon>
      </span>
    </div>
  }></List>;
}