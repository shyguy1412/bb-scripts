import { HSV, RGB, useSync } from "@/lib/components/ColorPicker";
import React, { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";

type Props = {
  hex: string,
  setRGB: (rgb: RGB) => void;
};


export function HexSelector({ hex, setRGB }: Props) {

  const [value, setValue] = useState(hex);
  const [intermediateValue, setIntermediateValue] = useState(hex);
  const [error, setError] = useState(false);
  const [focus, setFocus] = useState(false);

  const synced = useSync((v) => {
    console.log('SETTING', v, value);
    
    if (error) return;
    // if (v != value) setValue(v);
  }, [hex, value], true);

  console.log('render', { value, synced, error });

  useEffect(() => console.clear(), [focus]);

  useMemo(() => {
    console.log({ synced, value, focus });

    if (synced) return (console.log('set error 0'), setError(false));
    if (value.length != 3 && value.length != 6) return (console.log('set error 1', { error }), setError(true));

    const rgb = value.length == 3 ?
      {
        r: Number.parseInt(`${value[0]}${value[0]}`, 16),
        g: Number.parseInt(`${value[1]}${value[1]}`, 16),
        b: Number.parseInt(`${value[2]}${value[2]}`, 16),
      }
      : {
        r: Number.parseInt(value.slice(0, 2), 16),
        g: Number.parseInt(value.slice(2, 4), 16),
        b: Number.parseInt(value.slice(4, 6), 16),
      };

    if (Object.values(rgb).some(v => isNaN(v))) return (console.log('set error 2'), setError(true));

    (console.log('set error 3'), setError(false));
    setRGB(rgb);
  }, [value, focus]);

  console.log('');


  return <div
    style={{
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      borderBottom: '1px solid currentColor'
    }}
  >
    <span>HEX</span>
    <span># <input type="text"
      style={{
        width: '5em',
        appearance: 'none',
        MozAppearance: 'none',
        WebkitAppearance: 'none',
        border: 'none',
        background: 'transparent',
        color: 'inherit'
      }}
      value={value}
      onFocus={() => setFocus(true)}
      onBlur={() => setFocus(false)}
      onChange={({ currentTarget: { value } }) => value.length <= 6 && (console.log('change', { value }), setValue(value))}
    /></span>
  </div>;
}