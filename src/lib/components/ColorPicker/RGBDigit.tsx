import { useSync } from "@/lib/components/ColorPicker";
import React, { useEffect, useMemo, useState } from "react";

type Props = {
  digitLabel: 'R' | 'G' | 'B',
  digit: [number, (d: number) => void];
};

export function RGBDigit({ digitLabel, digit: [digit, setDigit] }: Props) {

  const [value, setValue] = useState(digit);
  const synced = useSync(setValue, [digit, value], digitLabel == 'R');

  // digitLabel == 'R' && console.log('render', { value, synced });

  useMemo(() => {
    // digitLabel == 'R' && console.log('memo', { value, synced });
    if (synced || digit == value) return;
    if (value > 255) return setValue(255);
    if (value < 0) return setValue(0);

    setDigit(value);
  }, [value]);

  return <div
    style={{
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      borderBottom: '1px solid currentColor'
    }}
  >
    <span>{digitLabel}</span>
    <input
      type="number"
      min='0'
      max='255'
      style={{
        width: '3.2em',
        appearance: 'none',
        MozAppearance: 'none',
        WebkitAppearance: 'none',
        border: 'none',
        background: 'transparent',
        color: 'inherit'
      }}
      value={value + ''}
      onChange={({ currentTarget: { value } }) => setValue(Number.parseInt((value || '0').replaceAll(/[^0-9]/g, '')))}
    /></div>;
}