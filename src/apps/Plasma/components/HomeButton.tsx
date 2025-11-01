//@ts-expect-error will need to wait until assertion typing
import LogoSVG from '../assets/bitburner-logo.svg' with {type: 'text'};
import { HomeMenu } from '../components/HomeMenu';
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
      <div dangerouslySetInnerHTML={{ __html: LogoSVG }}></div>
    </div>
    {!showMenu || <HomeMenu></HomeMenu>}
  </>;
}