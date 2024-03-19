import React from "react";
import { ReactNode, useState } from "react";

type Props = {
  value: string,
  options: string[];
  onChange: (value: string) => void;
};

export function DropDown({ value, options, onChange }: Props) {
  const [expanded, setExpanded] = useState(false);

  return <div onClick={
    () => {
      setExpanded(!expanded);
      // addEventListener('click', () => setExpanded(false), { once: true });
    }}>
    {value}
    {options.map(option => <div
      key={option}
      style={{
        display: expanded ? 'block' : 'none'
      }}
      onClick={({ currentTarget: { textContent } }) => onChange(textContent!)}
    >{option}</div>)}
  </div>;
}

function Option() {

}