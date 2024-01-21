import LogoSVG from '@/bb-plasma/assets/bitburner-logo.svg';
import { HomeMenu } from '@/bb-plasma/components/HomeMenu';
import React, { useState } from 'react';

type Props = {

};

export function HomeButton({ }: Props) {
  const [showMenu, setShowMenu] = useState(false);

  return <>
    <div className='taskbar-homebutton plasma-button plasma-box-inline'
      onClick={() => {
        setShowMenu(!showMenu);
        if (!showMenu) setTimeout(() => document.querySelector<HTMLDivElement>('.desktop-enviroment')
          ?.addEventListener('click', () => {
            setShowMenu(false);
          }, { once: true }), 10);
      }}>
      <LogoSVG></LogoSVG>
    </div>
    {!showMenu || <HomeMenu></HomeMenu>}
  </>;
}