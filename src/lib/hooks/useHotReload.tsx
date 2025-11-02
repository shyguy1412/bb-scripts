import { CleanupContext, NetscriptContext, TerminateContext } from '@/lib/Context';
import { useContext, useEffect, useMemo } from 'react';

const hmr_server_script = (file: string, pid: number) => /* javascript */`\
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

export function useHotReload() {
  const ns = useContext(NetscriptContext);
  const filename = useMemo(() => ns.self().filename, []);
  const hmrServer = useMemo(() => `.__hmr_server_${ns.pid}.js`, []);


  const terminate = useContext(TerminateContext);
  const addCleanup = useContext(CleanupContext);

  useEffect(() => {
    if (ns.isRunning(hmrServer)) return;
    console.log("Hot reload: " + filename);

    const controller = new AbortController();

    addCleanup(() => controller.abort());

    addEventListener('__hmr_hook_' + ns.pid, () => {
      terminate();
      addCleanup(() => ns.run(filename));
    }, { once: true, signal: controller.signal });

    ns.write(hmrServer, hmr_server_script(filename, ns.pid), 'w');
    ns.run(hmrServer);

    addCleanup(() => ns.kill(hmrServer));
    addCleanup(() => ns.rm(hmrServer));

  }, []);
}