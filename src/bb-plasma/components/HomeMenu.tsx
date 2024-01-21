import { RebootContext, TerminateContext, WindowManagerContext } from '@/bb-plasma/DesktopEnviroment';
import { faArrowsRotate, faPowerOff } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useContext } from 'react';

type Props = {

};

export function HomeMenu({ }: Props) {
  const { terminate } = useContext(TerminateContext);
  const { reboot } = useContext(RebootContext);

  const windowManager = useContext(WindowManagerContext);

  return <div className='homemenu plasma-box-inline'>

    <span className='plasma-button plasma-box-top' onClick={() => reboot()}>
      <FontAwesomeIcon icon={faArrowsRotate}></FontAwesomeIcon>
      <span className='plasma-center'>Reboot</span>
    </span>

    <span className='plasma-button plasma-box-top' onClick={() => terminate()}>
      <FontAwesomeIcon icon={faPowerOff}></FontAwesomeIcon>
      <span className='plasma-center'>Shutdown</span>
    </span>
  </div>;
}