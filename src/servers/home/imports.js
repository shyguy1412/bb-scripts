export async function main() {
  const module = await import("funky");
  console.log(module)
}

export function test() {
  return  "test";
}

export async function then(r) {
  const d = eval("document");
  const importmapEl = d.querySelector("[type=importmap]") || d.createElement("script");
  importmapEl.setAttribute("type", "importmap");
  d.head.append(importmapEl);
  const importmap = JSON.parse(importmapEl.textContent || "null") ?? {};

  importmap.imports ??= {};

  importmap.imports["funky"] = import.meta.url;

  importmapEl.textContent = JSON.stringify(importmap)

  await new Promise(r => setTimeout(r, 100));

  let { then, ...module } = this;

  r(module)
}
