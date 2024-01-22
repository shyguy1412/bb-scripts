import { sleep } from '@/bb-plasma/lib/Sleep';
import React from 'react';

type Props = {
  ns: NS;
};
export function Terminal({ ns }: Props) {

  // Acquire a reference to the terminal text field
  const terminalInput = document.getElementById('terminal-input');


  // Get a reference to the React event handler.
  const handler = Object.keys(terminalInput)[1];

  // Perform an onChange event to set some internal values.
  terminalInput[handler].onChange({ target: terminalInput });

  // // Simulate an enter press
  sleep(1000).then(async () => {
    console.log('ENTER');
    // Set the value to the command you want to run.
    terminalInput.value = 'home;connect n00dles;home;connect grindr-1;home;';

    await sleep(0);

    terminalInput[handler].onKeyDown({ key: 'Enter', preventDefault: () => null });
  });

  return <div>
    TERMINAL
  </div>;
}