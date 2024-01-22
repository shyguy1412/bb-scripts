import { Terminal } from '@/Terminal';
import { createWindowApp } from '@/lib/WindowApp';
import React, { } from 'react';

export async function main(ns: NS) {
  const WindowApp = createWindowApp(ns);

  return WindowApp.mount(<Terminal ns={ns}></Terminal>);
}
