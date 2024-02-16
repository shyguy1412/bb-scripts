import React from "react";

type Props = {
  accept: string | string[];
} & React.HTMLAttributes<HTMLDivElement>;

export function DropTarget({ accept, onDragEnter, onDragOver, ...attr }: Props) {
  const shouldAccept = (types: readonly string[]) => {
    const valid = typeof accept == 'string' ? [accept] : accept;
    return valid.some(v => types.includes(v));
  };
  return <div
    draggable={true}
    data-drag-accept={accept}
    onDragEnter={(e) => {
      if (shouldAccept(e.dataTransfer.types)) e.preventDefault();
      if (onDragEnter) onDragEnter(e);
    }}
    onDragOver={(e) => {
      if (shouldAccept(e.dataTransfer.types)) e.preventDefault();
      if (onDragOver) onDragOver(e);
    }}
    {...attr}
  ></div>;
}