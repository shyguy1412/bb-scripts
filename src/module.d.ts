declare module '*.css' {
  const Style: import('react').FunctionComponent;
  export default Style;
}

declare module '*.svg' {
  const content: string;
  export default content;
}