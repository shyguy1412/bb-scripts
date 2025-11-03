export function registerProcess(ns: NS) {
  const filename = ns.self().filename.replace(/.*\/([^\.]*).*/, "$1");
  const pidFile = `/run/${filename}.txt`;

  //0 RAM check if process is singleton
  if (ns.read(pidFile)) throw new Error("process is already registered");

  ns.write(pidFile, ns.pid + "", 'w');
  ns.atExit(() => {
    ns.ramOverride(ns.self().dynamicRamUsage! + 1);
    ns["rm"](pidFile);
  }, "__syscall_register_process");
}
