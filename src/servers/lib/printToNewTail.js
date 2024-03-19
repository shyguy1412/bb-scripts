/**
 * @param {NS} ns
 */
export async function main(ns) {
  console.log(await createNewTail(ns));
}

/**
 * @param {NS} ns
 */
async function createNewTail(ns) {
  /**
   * @param {NS} ns
   * @returns {HTMLDivElement}
   */
  function main(ns) {
    const el = document.createElement('div');
    el.id = `tail-${ns.pid}`;
    ns.printRaw(React.createElement('div', { ref: ref => ref?.append(el) }));
    ns.writePort(ns.pid, el.id);
  }

  ns.write('tail-spawner.js', `export ${main.toString()}`);

  const pid = ns.run('tail-spawner.js');
  await ns.nextPortWrite(pid);
  const id = ns.readPort(pid);
  ns.clearPort(pid);

  return document.getElementById(id);
}