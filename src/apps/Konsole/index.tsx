import { sleep } from '@/lib/System';
import Style from './Konsole.css' with {type: 'css'};
import { Terminal } from '@/lib/Terminal';
import React, { useContext, useEffect, useRef, useState } from 'react';
import { CleanupContext, NetscriptContext } from '@/lib/Context';

export function Konsole() {

  const ns = useContext(NetscriptContext);
  const terminal = new Terminal(ns);
  const addCleanup = useContext(CleanupContext);
  const konsoleRef = useRef<HTMLUListElement>(null);
  const [prompt, setPrompt] = useState('');
  const [command, setCommand] = useState('');
  const [disabled, setDisabled] = useState(!terminal.terminalInput);
  const [suggestions, setSuggestions] = useState<string[]>([]);

  useEffect(() => {
    const updateTerminal = async () => {
      if (!konsoleRef.current) return;
      konsoleRef.current.replaceChildren(...await terminal.getTerminalLines());
      setPrompt(terminal.terminalInput!.previousSibling?.textContent ?? '');
    };

    const watchTerminalLines = () => {
      const TerminalObserver = new MutationObserver(() => {
        updateTerminal();
      });
      TerminalObserver.observe(terminal.terminalElement!, { childList: true, subtree: true, characterData: true });
      addCleanup(() => TerminalObserver.disconnect());
    };

    terminal.addEventListener('connect', () => {
      watchTerminalLines();
      updateTerminal().then(() => {
        if (!konsoleRef.current) return;

        konsoleRef.current.scrollTop = konsoleRef.current.scrollHeight;
      }
      );
      setDisabled(false);
    });

    terminal.addEventListener('disconnect', () => {
      if (!konsoleRef.current) return;
      konsoleRef.current.textContent = 'Terminal is offline';
      setDisabled(true);
    });

    watchTerminalLines();

    sleep(0) //this gives up control so the terminal has time to display that this script started
      .then(() => updateTerminal())
      .then(() => {
        if (!konsoleRef.current) return;
        konsoleRef.current.scrollTop = konsoleRef.current.scrollHeight;
      }
      );

  }, []);

  addCleanup(() => terminal.cleanup());

  return <div className='konsole-terminal'>
    <Style></Style>
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
        value={disabled ? '[disconnected]' : command}
        autoComplete='off'
        spellCheck='false'
        onChange={({ currentTarget: { value } }) => setCommand(value)}
        disabled={!terminal.terminalInput}
        onKeyDown={async (e) => {
          if (disabled) return;
          if (e.key == 'Enter') {
            terminal
              .exec(command)
              .then(() => sleep(100))
              .then(() => {
                if (!konsoleRef.current) return;
                konsoleRef.current.scrollTop = konsoleRef.current.scrollHeight;
              }
              );
            setCommand('');
          }

          if (e.key == 'Tab') {
            if (e.altKey || e.ctrlKey) return;
            e.preventDefault();
            terminal
              .autoComplete(command)
              .then(suggestions => {
                if (suggestions.length == 1) {
                  setCommand(suggestions[0]);
                  setSuggestions([]);
                  return;
                };

                setSuggestions(suggestions);
                window.addEventListener('click', () => setSuggestions([]), { once: true });
              });
            return;
          }

          const actions = [
            "ArrowUp",
            "ArrowDown"
          ];

          if (!actions.includes(e.key)) return;

          e.preventDefault();
          await terminal.inputKey(e.key);
          setCommand(terminal.terminalInput!.value);
        }}
      />
    </div>
  </div>;
}