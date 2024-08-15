# FapUI - Fast and Practical UI for Bitburner

FapUI is a simple UI framwork to help you get started with custom UI without requiring any knowledge of React.

## Getting started

FapUI dynamically generates wrappers for HTML elements. To do so you first need to import `FapComponents` from FapUI. You can then destructure FapComponents into all the components you want to use.  
A FapComponent is any HTML tag but starting with an uppercase letter. So the component for a `div` element would be `Div`, button `Button` and so on.  
All methods of a component return the component itself. This allows all the methods to be chained after another.

```js
import { FapComponents } from 'FapUI.js';

const {
  Span
} = FapComponents;

export async function main(ns){
  ns.disableLog('ALL');
  ns.tail();
  ns.clearLog();

  ns.printRaw(

    Span('Hello World!')

  );

  return new Promise(_ => { });
}

```

## Event Listeners

To attach an event listener to a component you can chain your events on the component

```js
Button('Press me')
  .onClick((el) => { el.Content('Ouchie!'); })
  .onMouseEnter((el) => { el.Content('Oh oh...'); })
  .onMouseLeave((el) => { el.Content('Press me'); })
```

The event handler receives the element the event was triggered on as first argument and the ReactEvent that was triggered as second.

## Styling

For styling you can call the `Style` method. It takes an object describing a CSS Rule (in camel case) and its value

```js
Button('Press me')
  .Style({ background: 'black', border: '1px solid red', color: 'red' })
```

## Update Content

To update the content of a component you can call the `Content` method. This method takes a ReactNode as input. A ReactNode can be any primitive value or a JSX Element. It can also be an array of elements to render.

```js

const outputSpan = Span("No input yet")
  .Style({ background: 'white', color: 'black', display: 'block' });

Div([
  Input().Id('my-input').Style({ display: 'block' }),

  Button('Submit')
    .onClick(() => outputSpan.Content([
      "Input:",
      Br(),
      document.querySelector('#my-input')!.value
    ])),

  outputSpan,
])

```

## Setting Attributes

HTML Attributes like id, class, aria attributes or data attributes can also be set by calling their respective method. The Method is always the attribute in camel case.

```js

Input()
  .Id('my-input')
  .Class('custom-class')
  .AriaBrailleroledescription('brrr') 
  .DataCustomDataAttr('brrr')
  .Style({ display: 'block' }),

```
