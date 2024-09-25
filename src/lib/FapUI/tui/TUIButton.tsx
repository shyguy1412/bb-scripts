import { FapComponents } from "@/lib/FapUI";

export function TUIButton(label: string) {
  const { Span } = FapComponents;
  //┌┐└┘─│

  const BorderSpan = Span.Overload({
    Content(state, content) {
      if (typeof content != 'string') throw new TypeError('Content of TUIButton must be a string');

      const top = `┌${Array(content.length).fill('─').join('')}┐`;
      const middle = `│${content}│`;
      const bottom = `└${Array(content.length).fill('─').join('')}┘`;
      state.content = `${top}\n${middle}\n${bottom}`;
    },
    Style(state, style) {
      state.style = { ...style, display: 'inline-block' };
    },
  });

  return BorderSpan()
    .Style({})
    .Content(label);
}