import React from 'react';
import { createPortal } from 'react-dom';

export async function main(ns: NS) {
  const el = document.createElement('div');
  const parent = document.getElementById('root')!;

  el.textContent = 'HELLO WORLD';

  parent.prepend(el);
}
