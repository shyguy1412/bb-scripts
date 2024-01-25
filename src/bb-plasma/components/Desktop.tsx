import { FileGrid } from '@/bb-plasma/components/FileGrid';
import React, { useContext } from 'react';

type Props = {

};

export function Desktop({ }: Props) {

  return <div className='plasma-desktop'>
    <FileGrid></FileGrid>
  </div>;
}