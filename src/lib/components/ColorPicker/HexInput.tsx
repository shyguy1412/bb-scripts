import { RGB, useSyncState } from "./";
import React, { ChangeEvent, useMemo, useState } from "react";

type Props = {
  rgb: RGB,
  setRGB: (rgb: RGB) => void;
};


export function formatRGBtoHEX(rgb: RGB) {
  const digits = [
    rgb.r.toString(16).padStart(2, '0'),
    rgb.g.toString(16).padStart(2, '0'),
    rgb.b.toString(16).padStart(2, '0')
  ];

  if (digits.every(d => d[0] == d[1])) return `${digits[0][0]}${digits[1][0]}${digits[2][0]}`;
  return digits.join('');
}


export function HexInput({ rgb, setRGB }: Props) {

  const hex = formatRGBtoHEX(rgb);

  const [value, setValue] = useSyncState(hex);
  const [error, setError] = useState(false);

  useMemo(() => {
    if (value.length != 3 && value.length != 6) return error || setError(true);

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

    if (Object.values(rgb).some(v => isNaN(v))) return error || setError(true);
    error && setError(false);

    if (hex == formatRGBtoHEX(rgb)) return;

    setRGB(rgb);
  }, [value]);

  return <HexInputDisplay
    value={value}
    onChange={({ currentTarget: { value } }) => value.length <= 6 && setValue(value)}
    error={error}
  ></HexInputDisplay>;
}



type DisplayProps = {
  error: boolean;
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
};

function HexInputDisplay({ error, value, onChange }: DisplayProps) {
  return <div
    style={{
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      borderBottom: '1px solid currentColor'
    }}
  >
    <span style={{fontWeight: 'bolder'}}>HEX</span>
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
        onChange={onChange}
      />
      <span style={{ display: 'inline-block', width: '1em', textAlign: 'right' }}>{error ? 'X' : ''}</span>
    </span>
  </div>;
}