import React from "react";
import ReactDOM from "react-dom";

export async function main(ns: NS) {
  const hook0 = document.getElementById('overview-extra-hook-0');

  if (!hook0) throw new Error("Could not get hook");

  ReactDOM.render(<OverviewComponent></OverviewComponent>, hook0);

  ns.atExit(() => {
    ReactDOM.unmountComponentAtNode(hook0);
  });

  return new Promise(() => { });
}

function OverviewComponent() {
  return <div>My hook!</div>;
}