import LogoSVG from '@/Plasma/assets/bitburner-logo.svg';
import { HomeMenu } from '@/Plasma/components/HomeMenu';
import React, { useState } from 'react';

type Props = {

};

export function HomeButton({ }: Props) {
  const [showMenu, setShowMenu] = useState(false);
  
  return <>
    <div className='taskbar-homebutton plasma-button plasma-box-inline'
      onClick={() => {
        setShowMenu(!showMenu);
        if (!showMenu) setTimeout(() => document.querySelector<HTMLHtmlElement>('html')
          ?.addEventListener('click', () => {
            setShowMenu(false);
          }, { once: true }), 0);
      }}>
      <LogoSVG></LogoSVG>
    </div>
    {!showMenu || <HomeMenu></HomeMenu>}
  </>;
}