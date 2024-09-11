import { BatchManager } from "@/apps/BatchManager";
import { createWindowApp } from "@/lib/WindowApp";
import React from "react";

export function main(ns: NS) {
  const windowApp = createWindowApp(ns);

  ns.atExit(() => windowApp.cleanup());

  return windowApp.mount(<BatchManager></BatchManager>);
}