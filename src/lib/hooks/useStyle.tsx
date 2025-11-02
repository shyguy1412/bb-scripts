import { CleanupContext } from "@/lib/Context";
import { useContext } from "react";

export function useStyle(style: CSSStyleSheet) {
  if (document.adoptedStyleSheets.includes(style)) return;

  document.adoptedStyleSheets.push(style);

  const addCleanup = useContext(CleanupContext);

  addCleanup(() => {
    document.adoptedStyleSheets = document.adoptedStyleSheets.filter(sheet => sheet != style);
  });
}