import React from "react";

type Props = {
  accept: string;
} & React.HTMLAttributes<HTMLDivElement>;

export function DropTarget({ accept, onDragEnter, onDragOver, ...attr }: Props) {
  return <div
    draggable={true}
    data-drag-accept={accept}
    onDragEnter={(e) => {
      if (e.dataTransfer.types.includes(accept)) e.preventDefault();
      if (onDragEnter) onDragEnter(e);
    }}
    onDragOver={(e) => {
      if (e.dataTransfer.types.includes(accept)) e.preventDefault();
      if (onDragOver) onDragOver(e);
    }}
    {...attr}
  ></div>;
}