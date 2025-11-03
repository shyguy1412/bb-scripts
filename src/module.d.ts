declare module '*.css' {
  const content: CSSStyleSheet;
  export default content;
}

declare module 'meta:filename' {
  const content: string;
  export default content;
}

declare module 'meta:entrypoint' {
  const content: string;
  export default content;
}
// declare module '*.svg' {
//   const content: string;
//   export default content;
// }