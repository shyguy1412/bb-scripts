import { HSV, RGB, useSync } from "@/lib/components/ColorPicker";
import React, { Dispatch, MouseEvent, SetStateAction, useEffect, useState } from "react";

type Props = {
  sat: number;
  val: number;
  hue: number;
  setSV: (sv: Omit<HSV, 'hue'>) => void;
};

const MARKER_RADIUS = 4;

function constrain(val: number, min: number, max: number) {
  return val > min ? (val < max ? val : max) : min;
}

export function SVCanvas({ hue, sat, val, setSV }: Props) {

  const [s, setS] = useState(sat);
  const [v, setV] = useState(val);

  const sSync = useSync(setS, [sat, s]);
  const vSync = useSync(setV, [val, v]);
  const sync = sSync || vSync;

  // const [marker, setMarker] = useState({ x: Math.round((sat / 100) * 320), y: Math.round(((100 - val) / 100) * 128) });

  const setSVByEvent = (event: MouseEvent) => {
    const pos = event.currentTarget.getBoundingClientRect();
    const x = constrain(event.clientX - pos.x - MARKER_RADIUS, 0 - MARKER_RADIUS, 320 - MARKER_RADIUS);
    const y = constrain(event.clientY - pos.y - MARKER_RADIUS, 0 - MARKER_RADIUS, 128 - MARKER_RADIUS);
    const s = (x + MARKER_RADIUS) / 320 * 100;
    const v = 100 - (y + MARKER_RADIUS) / 128 * 100;
    setS(s);
    setV(v);
  };

  // const setMarkerByEvent = (event: MouseEvent) => {
  // };

  useEffect(() => {
    if (sync) return;
    setSV({ sat: Math.floor(s), val: Math.floor(v) });
  }, [s, v]);

  return <div
    style={{
      width: '100%',
      height: '128px',
      backgroundColor: `hsl(${hue} 100% 50%)`
    }}
    draggable={false}
    onMouseDown={(e) => (setSVByEvent(e))}
    onMouseMove={(e) => e.buttons == 1 && (setSVByEvent(e))}
  >
    <div
      style={{
        width: '100%',
        height: '100%',
        background: 'rgba(0, 0, 0, 0) linear-gradient(to right, rgb(255, 255, 255), rgba(255, 255, 255, 0)) repeat scroll 0% 0%'
      }}
    >
      <div
        style={{
          width: '100%',
          height: '100%',
          background: 'rgba(0, 0, 0, 0) linear-gradient(to top, rgb(0, 0, 0), rgba(0, 0, 0, 0)) repeat scroll 0% 0%',
          position: 'relative'
        }}
      >
        <Marker s={s} v={v}></Marker>
      </div>
    </div>
  </div>;
}


type MarkerProps = {
  s: number,
  v: number;
};

function Marker({ s, v }: MarkerProps) {
  return <div
    style={{
      width: `${MARKER_RADIUS * 2}px`,
      height: `${MARKER_RADIUS * 2}px`,
      background: 'transparent',
      border: '1.2px solid white',
      borderRadius: '100%',
      position: 'absolute',
      left: `calc(${s}% - ${MARKER_RADIUS}px)`,
      bottom: `calc(${v}% - ${MARKER_RADIUS}px)`,
    }}
  ></div>;
}