# WindowApp

A window app is a tail window that contains a react app. It provides context for common things too.  
WindowApp.mount will return a promise that resolves when the tail window is closed. This can be used to cleanly exit a script after tail was closed

## How to use

```jsx
import { createWindowApp } from 'WindowApp';

export async function main(ns) {
  const WindowApp = createWindowApp(ns);

  ns.atExit(() => { //this will cleanup the window and run all registered cleanups in the cleanup context
    WindowApp.cleanup();
  });

  return WindowApp.mount(<MyApp></MyApp>); //this will mount the MyApp component in the tail window. 
}
```

## Contexts

### NetscriptContext

Use this to get the ns instance of your app

```jsx
import {NetscriptContext} from 'WindowApp';
function MyApp(){

  const ns = useContext(NetscriptContext);

  return <div>My App</div>
}
```

### CleanupContext

Use this to add callbacks to be executed when the script dies

```jsx
import {CleanupContext} from 'WindowApp';

function MyApp(){

  const addCleanup = useContext(CleanupContext);

  addCleanup(() => {/* add stuff here */});

  return <div>My App</div>
}
```

### TerminateContext

Use this to cleanly exit the script. Unlike ns.exit, this will not stop immediate execution.

```jsx
import {TerminateContext} from 'WindowApp';

function MyApp(){

  const terminate = useContext(TerminateContext);

  terminate(); //Script will exit after the current task is done.

  return <div>My App</div>
}
```
