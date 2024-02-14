import { TerminateContext } from '@/lib/Context';
import { faPowerOff } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';

type Props = {

};

export function HomeMenu({ }: Props) {
  const terminate = useContext(TerminateContext);

  return <div className='homemenu plasma-box-inline'>
    <span className='plasma-button plasma-box-top' onClick={() => terminate()}>
      <FontAwesomeIcon icon={faPowerOff}></FontAwesomeIcon>
      <span className='plasma-center'>Shutdown</span>
    </span>
  </div>;
}