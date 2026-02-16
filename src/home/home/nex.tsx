import React, { useState } from "react";
import { useEffect } from "react";

export async function main(ns:NS) {
  ns.tprintRaw(<Component></Component>)
  return new Promise(() => {});
}

function Component(){
  const [state, setState] = useState(false);
  console.log();
  useEffect(() => {
    console.log("effect")
    setTimeout(() => setState(state => !state))
  }, true);
  return <div>Hi; {state?"true":"false"}</div>
}