import { hotReload } from "@/lib/syscalls/HotReload";
import { createProcessInterface } from "@/lib/syscalls/ProcessInterface";
import { registerProcess } from "@/lib/syscalls/RegisterProcess";
import { getSafePortHandle } from "@/lib/System";
import __META_FILENAME from "meta:filename";

type FSEvent = {
  type: "change",
  filename: string;
  content: string;
};

type SubscribeMessage = {
  action: "subscribe",
  filename: string,
  listener: number,
  event: FSEvent["type"];
};

type UnsubscribeMessage = {
  action: "unsubscribe",
  filename: string,
  listener: number,
  event: FSEvent["type"];
};

type FSDaemonMessage = SubscribeMessage | UnsubscribeMessage;

export const connect = createProcessInterface<
  FSDaemonMessage,
  FSEvent
>(__META_FILENAME);

type FsDaemonData = {
  listeners: Map<string, Map<number, SubscribeMessage>>;
  fileCache: Map<string, string>;
};

export async function main(ns: NS) {
  registerProcess(ns);

  hotReload(ns);

  const daemonData = {
    listeners: new Map<string, Map<number, SubscribeMessage>>(),
    fileCache: new Map<string, string>(),
  };

  while (true) {
    await ns.sleep(0);

    for (const [filename, content] of daemonData.fileCache.entries()) {
      const fileListeners = daemonData.listeners.get(filename);
      if (!fileListeners) {
        daemonData.fileCache.delete(filename);
        continue;
      };

      const deadListeners = fileListeners.keys().filter(pid => !ns.isRunning(pid));
      for (const deadListener of deadListeners) fileListeners.delete(deadListener);

      const newContent = ns.read(filename);

      if (content == newContent) continue;
      daemonData.fileCache.set(filename, newContent);

      const event: FSEvent = {
        type: "change",
        filename,
        content: newContent
      };

      for (const [pid] of fileListeners) {
        ns.writePort(pid, {
          process: __META_FILENAME,
          payload: event,
        });
      }
    }

    emptyRequestQueue(ns, daemonData);
  }
}

function emptyRequestQueue(ns: NS, data: FsDaemonData) {
  const port = getSafePortHandle(ns, ns.pid)!;

  while (!port.empty()) {
    const msg: FSDaemonMessage = port.read();
    //ignore broken files
    try { ns.read(msg.filename); } catch { continue; }
    if (msg.action == "subscribe") {
      const fileListeners = data.listeners.get(msg.filename) ?? new Map();
      data.listeners.set(msg.filename, fileListeners);
      fileListeners.set(msg.listener, msg);
      data.fileCache.set(msg.filename, ns.read(msg.filename));
    } else {
      const fileListeners = data.listeners.get(msg.filename);
      if (!fileListeners) continue;
      fileListeners.delete(msg.listener);
    }
  }
}