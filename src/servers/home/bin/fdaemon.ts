import { hotReload } from "@/lib/syscalls/HotReload";
import { createProcessInterface } from "@/lib/syscalls/ProcessInterface";
import { registerProcess } from "@/lib/syscalls/RegisterProcess";
import { getSafePortHandle } from "@/lib/System";
import metaFilename from "meta:filename";

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
>(metaFilename);

export async function main(ns: NS) {
  registerProcess(ns);

  // hotReload(ns);

  const port = getSafePortHandle(ns, ns.pid)!;

  const listeners = new Map<string, Map<number, SubscribeMessage>>();
  const fileCache = new Map<string, string>();

  let garbageCollector = setInterval(() => {
    const watchedFiles = listeners.keys().toArray();
    const filesToDelete = fileCache.keys().filter(file => !watchedFiles.includes(file));
    for (const fileToDelete of filesToDelete) fileCache.delete(fileToDelete);
  }, 500);

  ns.atExit(() => clearInterval(garbageCollector));

  while (true) {
    await ns.sleep(500);

    for (const [filename, content] of fileCache.entries()) {
      const newContent = ns.read(filename);

      if (content == newContent) continue;
      fileCache.set(filename, newContent);

      const fileListeners = listeners.get(filename);

      if (!fileListeners) continue;

      //before dispatch clear dead listeners
      const deadListeners = fileListeners.keys().filter(pid => !ns.isRunning(pid));
      for (const deadListener of deadListeners) fileListeners.delete(deadListener);

      const event: FSEvent = {
        type: "change",
        filename,
        content: newContent
      };

      for (const [pid] of fileListeners) {
        ns.writePort(pid, {
          process: metaFilename,
          payload: event,
        });
      }
    }

    while (!port.empty()) {
      const msg: FSDaemonMessage = port.read();
      if (msg.action == "subscribe") {
        const fileListeners = listeners.get(msg.filename) ?? new Map();
        listeners.set(msg.filename, fileListeners);
        fileListeners.set(msg.listener, msg);
        fileCache.set(msg.filename, ns.read(msg.filename));
      } else {
        const fileListeners = listeners.get(msg.filename);
        if (!fileListeners) continue;
        fileListeners.delete(msg.listener);
      }
    }
  }
}