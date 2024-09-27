import { ColorPreview } from "@/lib/components/ColorPicker/ColorPreview";
import { HexSelector } from "@/lib/components/ColorPicker/HexSelector";
import { HueSlider } from "@/lib/components/ColorPicker/HueSlider";
import { RGBDigit } from "@/lib/components/ColorPicker/RGBDigit";
import { SVCanvas } from "@/lib/components/ColorPicker/SVCanvas";
import { useReload } from "@/lib/hooks/useReload";
import React, { useState } from "react";

export type HSV = {
  hue: number;
  sat: number;
  val: number;
};

export type RGB = {
  r: number;
  g: number;
  b: number;
};

type Props = {
  initialColor: RGB | HSV;
} & React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;

function normalize(color: HSV | RGB) {
  const isRGB = (c: RGB | HSV): c is RGB => Object.hasOwn(color, 'r');
  const rgb: RGB = isRGB(color) ? color : HSVtoRGB(color);
  const hsv: HSV = !isRGB(color) ? color : RGBtoHSV(color);
  return { rgb, hsv };
}

export function ColorPicker({ initialColor, ...attr }: Props) {
  useReload();

  const [color, setColor] = useState<RGB | HSV>(initialColor ?? { r: 50, g: 20, b: 50 });

  const { rgb, hsv } = normalize(color);

  return <div
    style={{
      width: '320px',
      display: 'flex',
      flexDirection: 'column'
    }}
    onKeyDown={(e) => { e.stopPropagation(); }}
  >
    <SVCanvas hue={hsv.hue} sat={hsv.sat} val={hsv.val} setSV={(sv) => setColor({ hue: hsv.hue, ...sv })}></SVCanvas>
    <HueSlider hue={hsv.hue} setHue={(h) => setColor({ hue: h, sat: hsv.sat, val: hsv.val })}></HueSlider>
    <span
      style={{
        display: 'flex',
        justifyContent: 'space-between'
      }}
    >
      <ColorPreview rgb={rgb} ></ColorPreview>
      <HexSelector hex={formatRGBtoHEX(rgb)} setRGB={(c) => setColor(c)}></HexSelector>
      <RGBDigit digitLabel="R" digit={[rgb.r, r => setColor({ ...rgb, r })]}></RGBDigit>
      <RGBDigit digitLabel="G" digit={[rgb.g, g => setColor({ ...rgb, g })]}></RGBDigit>
      <RGBDigit digitLabel="B" digit={[rgb.b, b => setColor({ ...rgb, b })]}></RGBDigit>
    </span>
  </div >;
}

export function useSync<T>(set: (arg: T) => void, [toSet, toSync]: [T, T], l = false): boolean {
  const [prev, setPrev] = useState(toSync);
  l && console.log('SYNCING', { toSet, toSync, prev });
  
  if (!Object.is(prev, toSet)) {
    l && console.log('SYNCING TO', { toSet });
  
    setPrev(toSet);
    set(toSet);
    return true;
  }

  if (Object.is(toSync, toSet)) {
    l && console.log('ALREADY SYNCED');
    return true;
  }
  
  if (Object.is(prev, toSet)) return false;
  

  // if (prev == toSet) {
  //   return false;
  // }
  // l && console.log('SYNCING TO', { toSet });
  // set(toSet);
  // setPrev(toSet);
  return false;
}

function formatRGBtoHEX(rgb: RGB) {
  const digits = [
    rgb.r.toString(16).padStart(2, '0'),
    rgb.g.toString(16).padStart(2, '0'),
    rgb.b.toString(16).padStart(2, '0')
  ];

  if (digits.every(d => d[0] == d[1])) return `${digits[0][0]}${digits[1][0]}${digits[2][0]}`;
  return digits.join('');
}

function HSVtoRGB({ hue, sat, val }: HSV): RGB {
  const h = hue;
  const s = sat / 100;
  const v = val / 100;
  // https://stackoverflow.com/questions/17242144/javascript-convert-hsb-hsv-color-to-rgb-accurately
  let f = (n: number, k = (n + h / 60) % 6) => v - v * s * Math.max(Math.min(k, 4 - k, 1), 0);
  return {
    r: Math.round(f(5) * 255),
    g: Math.round(f(3) * 255),
    b: Math.round(f(1) * 255)
  };
}

function RGBtoHSV(rgb: RGB): HSV {
  // https://stackoverflow.com/questions/3018313/algorithm-to-convert-rgb-to-hsv-and-hsv-to-rgb-in-range-0-255-for-both
  const hsv: HSV = { hue: 0, sat: 0, val: 0 };
  let min, max, delta;

  const r = rgb.r / 255;
  const g = rgb.g / 255;
  const b = rgb.b / 255;

  min = r < g ? r : g;
  min = min < b ? min : b;

  max = r > g ? r : g;
  max = max > b ? max : b;

  hsv.val = Math.floor(max * 100);                                // v
  delta = max - min;


  if (delta < 0.00001) {
    hsv.sat = 0;
    hsv.hue = 0; // undefined, maybe nan?
    return hsv;
  }
  if (max > 0.0) { // NOTE: if Max is == 0, this divide would cause a crash
    hsv.sat = Math.floor((delta / max) * 100);                  // s
  } else {
    // if max is 0, then r = g = b = 0              
    // s = 0, h is undefined
    hsv.sat = 0.0;
    hsv.hue = 0;                            // its now undefined
    return hsv;
  }
  if (r >= max)                           // > is bogus, just keeps compilor happy
    hsv.hue = (g - b) / delta;        // between yellow & magenta
  else
    if (g >= max)
      hsv.hue = 2.0 + (b - r) / delta;  // between cyan & yellow
    else
      hsv.hue = 4.0 + (r - g) / delta;  // between magenta & cyan

  hsv.hue *= 60.0;                              // degrees

  if (hsv.hue < 0.0)
    hsv.hue += 360.0;

  hsv.hue = Math.floor(hsv.hue);

  return hsv;
}