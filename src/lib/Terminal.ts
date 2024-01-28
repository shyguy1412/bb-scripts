import { watchElForDeletion, watchSelectorForCreation } from "@/lib/BitburnerDOM";
import { sleep } from "@/lib/System";

export type TerminalCommands = "scan-analyze" |
  "alias" |
  "analyze" |
  "backdoor" |
  "buy" |
  "cat" |
  "cd" |
  "changelog" |
  "check" |
  "clear" |
  "cls" |
  "connect" |
  "cp" |
  "download" |
  "expr" |
  "free" |
  "grow" |
  "hack" |
  "help" |
  "history" |
  "home" |
  "hostname" |
  "kill" |
  "killall" |
  "ls" |
  "lscpu" |
  "mem" |
  "mv" |
  "nano" |
  "ps" |
  "rm" |
  "run" |
  "scan" |
  "scp" |
  "sudov" |
  "tail" |
  "apr1" |
  "top" |
  "unalias" |
  "vim" |
  "weaken" |
  "wget";

export class Terminal extends EventTarget {

  cleanupMap: Record<string, () => void> = {};
  terminalInput: HTMLInputElement = null;
  terminalElement: HTMLUListElement = null;
  #terminalInputProps: string = null;
  #awaitConnectQueue: (() => void)[] = [];

  constructor(ns: NS) {
    super();
    const onDisconnect = () => {

      const { cleanup } = watchSelectorForCreation('#terminal-input', () => {
        onConnect();
        this.dispatchEvent(new Event('connect'));
      });

      if (this.cleanupMap['deletion']) this.cleanupMap['deletion']();
      this.cleanupMap['deletion'] = cleanup;
    };

    const onConnect = () => {
      this.terminalInput = document.querySelector('#terminal-input');
      this.terminalElement = document.querySelector('#terminal');
      this.#terminalInputProps = Object.keys(this.terminalInput).find(k => k.includes('__reactProps'));
      while (this.#awaitConnectQueue.length) this.#awaitConnectQueue.shift()();

      const { cleanup } = watchElForDeletion(this.terminalInput, () => {
        this.terminalInput = null;

        onDisconnect();
        this.dispatchEvent(new Event('disconnect'));
      });

      if (this.cleanupMap['creation']) this.cleanupMap['creation']();
      this.cleanupMap['creation'] = cleanup;
    };

    this.terminalInput = document.querySelector('#terminal-input');
    if (this.terminalInput) onConnect();
    else onDisconnect();
  }

  async connect() {
    if (!this.terminalInput)
      await new Promise<void>(resolve => this.#awaitConnectQueue.push(() => resolve()));
  }

  async exec(cmd: string) {
    await this.connect();

    this.terminalInput.value = cmd;
    this.terminalInput[this.#terminalInputProps].onChange({ target: this.terminalInput });
    this.terminalInput[this.#terminalInputProps].onKeyDown({ key: 'Enter', preventDefault: () => null });

  }

  async inputKey(key: string) {
    await this.connect();
    this.terminalInput[this.#terminalInputProps].onKeyDown({ key, preventDefault: () => null });
  }

  async getTerminalLines(): Promise<Element[]> {
    await this.connect();
    return [...this.terminalElement.children].map(c => c.cloneNode(true) as Element);
  }

  async autoComplete(cmd: string, suppressTooltip = true) {
    await this.connect();

    const { cleanup } = watchSelectorForCreation('div[role="tooltip"]', (el) => {
      if (suppressTooltip) el.style.display = 'none';//hide tooltip before it gets rendered
    });

    setTimeout(() => cleanup(), 500); //make sure we always clean up

    this.terminalInput.value = cmd;
    this.terminalInput[this.#terminalInputProps].onChange({ target: this.terminalInput });
    this.terminalInput[this.#terminalInputProps].onKeyDown({ key: 'Tab', preventDefault: () => null });

    await sleep(0); //let handlers do their thing and update values

    const tooltip = [...document.querySelectorAll<HTMLDivElement>('div[role="tooltip"]')]
      .find(d => d.innerHTML.includes('Possible autocomplete candidates:'));

    const autoComplete = tooltip ?
      tooltip.firstElementChild.lastElementChild.textContent.split(' ') :
      [this.terminalInput.value];

    return autoComplete;
  }

  cleanup() {
    for (const key in this.cleanupMap) this.cleanupMap[key]();
  }
}