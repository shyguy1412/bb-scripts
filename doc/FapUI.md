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

## Binding a Component

Binding allows you to update and read the state of components more easily. A component is any function that returns a FapElement.  
Lets assume we have this simple component and want to increase the number displayed by 1 every time the `div` is clicked.

```js

function Counter(){
  return Div(0)
}

```

First we create a binding for the `Div` with the `createBinding` function.
The first parameter of `createBinding` is the component you want to bind, the rest are the arguments you want to bind.
It then returns an array containing the bound component and a getter/setter tuple for each bound argument.
The `Div` component only takes one argument so that would look like this.

```js

function Counter(){
  const [BoundDiv, [getCount, setCount]] = createBinding(Div, 0)
  return BoundDiv
}


```

For a component with more than one argument your code may look like this

```js

const [
  BoundComponent,
  [getA, setA],
  [getB, setB],
  [getC, setC],
] = createBinding(SomeComponent, a, b, c);

```

After a component was bound its locked in. This means event listeners, style and attributes have to be set before the component is bound.

```js

function Counter(){

  const WrappedDiv = (count) => Div(count)
    .Style({ 'background': 'green' })
    .onClick(() => setCount(getCount() + 1));

  const [BoundDiv, [getCount, setCount]] = createBinding(WrappedDiv, 0)
  return BoundDiv;
}

```

This completes a simple counter component that can be rendered directly

```js

ns.printRaw(Counter())

```

## Fragments

A Fragment is a logical container for elements. It combines a list of elements into a single one without wrapping them into any HTML.
This example renders 3 span elements without any container.

```js

ns.printRaw(Fragment([
  Span(1), Span(2), Span(3)
]));

```
