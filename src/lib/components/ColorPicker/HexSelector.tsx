import { RGB, useSync } from "@/lib/components/ColorPicker";
import React, { useEffect, useMemo, useState } from "react";

type Props = {
  hex: string,
  setRGB: (rgb: RGB) => void;
};


export function HexSelector({ hex, setRGB }: Props) {

  const [value, setValue] = useState(hex);
  const [error, setError] = useState(false);
  const [focus, setFocus] = useState(false);

  const synced = useSync(setValue, [hex, value], true);

  useEffect(() => console.clear(), [focus]);

  useMemo(() => {
    if (synced) return setError(false);
    if (value.length != 3 && value.length != 6) return setError(true);

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

    if (Object.values(rgb).some(v => isNaN(v))) return setError(true);

    setError(false);
    setRGB(rgb);
  }, [value, focus]);


  return <div
    style={{
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      borderBottom: '1px solid currentColor'
    }}
  >
    <span>HEX</span>
    <span
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        gap: '0.3em'
      }}
    >
      <span>#</span>
      <input type="text"
        style={{
          width: '3.5em',
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
        onChange={({ currentTarget: { value } }) => value.length <= 6 && setValue(value)}
      />
      <span style={{ display: 'inline-block', width: '1em', textAlign: 'right' }}>{error ? 'X' : ''}</span>
    </span>
  </div>;
}