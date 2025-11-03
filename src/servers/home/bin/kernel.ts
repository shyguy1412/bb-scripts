// must have kernel features:
// filesystem events
// pipes

import { registerProcess } from "@/lib/syscalls/RegisterProcess";
import { getSafePortHandle } from "@/lib/System";

const syscalls = {
  "hmr": () => { },
} as const;

export type Syscall = keyof typeof syscalls;

export async function main(ns: NS) {
  registerProcess(ns);

  const port = getSafePortHandle(ns, ns.pid);

  while (true) {
    if (port.empty())
      await port.nextWrite();

    const msg = port.read();

    if(msg in syscalls){
      syscalls[msg as Syscall]();
    }
  }
}