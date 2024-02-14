import { NetscriptContext } from '@/lib/Context';
import { readDir } from '@/lib/FileSystem';
import { FileGrid } from '@/lib/components/FileGrid';
import React, { useContext, useEffect, useState } from 'react';

export function Desktop() {

  const ns = useContext(NetscriptContext);

  const [_, reload] = useState(true); //this is just used to poll the fs since BB doesnt have fs events
  useEffect(() => {
    const timeout = setTimeout(() => reload(!_), 500); //just swapping between true/false
    return () => clearTimeout(timeout);
  });

  return <div className='plasma-desktop'>
    <FileGrid path={'home'} files={readDir(ns, 'home')} ></FileGrid>
  </div>;
}