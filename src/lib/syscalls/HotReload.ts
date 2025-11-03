const hmrServerScript = (file: string, pid: number) => /* javascript */`\
export async function main(ns){
  const original_content = ns.read('${file}'); 
  while(true){
    await ns.sleep(0);
    if(ns.read('${file}') == original_content) continue;
    dispatchEvent(new Event('__hmr_hook_' + ${pid}));
    break;
  }
}
`;

export function hotReload(ns: NS) {
  const filename = ns.self().filename;
  const hmrServer = `/tmp/.__hmr_server_${ns.pid}.js`;
  if (ns.isRunning(hmrServer)) return;

  const cleanup: (() => void)[] = [];
  const addCleanup = (f: () => void) => cleanup.push(f);
  ns.atExit(() => cleanup.forEach(c => c()), '__syscall_hmr_cleanup_hook');

  const controller = new AbortController();

  addCleanup(() => controller.abort());

  addEventListener('__hmr_hook_' + ns.pid, () => {
    console.log("Hot reload: " + filename);
    addCleanup(() => ns.run(filename));
    try {
      //swallow any concurrency errors
      ns.exit();
    } catch { };
  }, { once: true, signal: controller.signal });

  ns.write(hmrServer, hmrServerScript(filename, ns.pid), 'w');
  ns.run(hmrServer);

  addCleanup(() => ns.kill(hmrServer));
  addCleanup(() => ns.rm(hmrServer));
}