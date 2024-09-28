import { ColorPreview } from "./ColorPreview";
import { HexInput } from "./HexInput";
import { HueSlider } from "./HueSlider";
import { RGBDigit } from "./RGBDigit";
import { SVCanvas } from "./SVCanvas";
import React, { Dispatch, useMemo, useReducer, useState } from "react";

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
  const [color, setColor] = useState<RGB | HSV>(initialColor ?? { r: 255, g: 255, b: 255 });

  const { rgb, hsv } = normalize(color);

  return <div
    style={{
      width: '320px',
      display: 'flex',
      flexDirection: 'column',
      userSelect: 'none'
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
      <HexInput rgb={rgb} setRGB={(c) => setColor(c)}></HexInput>
      <RGBDigit digitLabel="R" digit={[rgb.r, r => setColor({ ...rgb, r })]}></RGBDigit>
      <RGBDigit digitLabel="G" digit={[rgb.g, g => setColor({ ...rgb, g })]}></RGBDigit>
      <RGBDigit digitLabel="B" digit={[rgb.b, b => setColor({ ...rgb, b })]}></RGBDigit>
    </span>
  </div >;
}

function useDetachedState<T>(val: T) {
  //the useMemo are used like useState that dont trigger a rerender
  return useMemo(() => {
    const state = [val];
    return [state, (val: T) => { state[0] = val; }] as const;
  }, []);
}

export function useSyncState<T>(val: T): [T, Dispatch<T>] {
  
  //used to manually trigger a rerender on state change
  const [, rerender] = useReducer(() => ({}), {});
  
  //this hook internal state is irrelevant for rendering so we dont want to rerender on change
  const [[prev], setPrev] = useDetachedState(val);
  const [[state], setStateInternal] = useDetachedState(val);

  const setState = (value: T) => (setStateInternal(value), rerender());

  if (!Object.is(prev, val)) {
    setPrev(val);
    setStateInternal(val);
    return [val, setState];
  }

  return [state, setState];
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