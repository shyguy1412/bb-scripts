import { useReload } from "@/lib/hooks/useReload";
import React, {  } from "react";

export function GraphBurner() {
  useReload();

  return <div
    style={{ width: '100%', height: '100%', color: '--primary' }}>
  </div>;
}
