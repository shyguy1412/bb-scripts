import { NetworkExplorer } from "@/apps/NetworkExplorer";
import { createWindowApp } from "@/lib/WindowApp";
import React from "react";

export function main(ns: NS) {
  const WindowApp = createWindowApp(ns);

  ns.atExit(() => WindowApp.cleanup());

  return WindowApp.mount(<NetworkExplorer></NetworkExplorer>);
}