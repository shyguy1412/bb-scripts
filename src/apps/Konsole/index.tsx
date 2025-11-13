import { sleep } from '@/lib/System';
import style from './Konsole.css' with {type: 'css'};
import React, { KeyboardEventHandler, useCallback, useContext, useEffect, useRef, useState } from 'react';
import { CleanupContext, NetscriptContext } from '@/lib/Context';
import { adoptStyle } from '@/lib/hooks/useStyle';

export function Konsole() {

  const ns = useContext(NetscriptContext);
  const addCleanup = useContext(CleanupContext);

  const konsoleRef = useRef<HTMLUListElement>(null);

  const [prompt, setPrompt] = useState('');
  const [command, setCommand] = useState('');
  // const [disabled, setDisabled] = useState(!terminal.terminalInput);
  const [suggestions, setSuggestions] = useState<string[]>([]);

  // const terminal = useMemo(() => new Terminal(ns), [ns]);

  adoptStyle(style);

  useEffect(() => {
    const updateTerminal = async () => {
      if (!konsoleRef.current) return;
      // konsoleRef.current.replaceChildren(...await terminal.getTerminalLines());
      // setPrompt(terminal.terminalInput!.previousSibling?.textContent ?? '');
    };

    const watchTerminalLines = () => {
      const TerminalObserver = new MutationObserver(() => {
        updateTerminal();
      });
      // TerminalObserver.observe(terminal.terminalElement!, { childList: true, subtree: true, characterData: true });
      addCleanup(() => TerminalObserver.disconnect());
    };

    watchTerminalLines();

    sleep(0) //this gives up control so the terminal has time to display that this script started
      .then(() => updateTerminal())
      .then(() => {
        if (!konsoleRef.current) return;
        konsoleRef.current.scrollTop = konsoleRef.current.scrollHeight;
      }
      );

  }, []);

  const onKeyDown: KeyboardEventHandler<HTMLInputElement> = useCallback(async (e) => {

  }, []);

  return <div className='konsole-terminal'>
    <ul className='konsole-terminal-content' ref={konsoleRef}></ul>
    <div className='konsole-input-wrapper'>
      {suggestions.length ?
        <div className='konsole-suggestion-tooltip'>
          <div>
            Possible autocomplete canditates:
          </div>
          <div>
            {suggestions.join(' ')}
          </div>
        </div>
        : null}
      <span>{prompt}</span>
      <input
        type="text"
        className='konsole-input'
        // value={disabled ? '[disconnected]' : command}
        autoComplete='off'
        spellCheck='false'
        onChange={({ currentTarget: { value } }) => setCommand(value)}
        // disabled={!terminal.terminalInput}
        onKeyDown={onKeyDown}
      />
    </div>
  </div>;
}