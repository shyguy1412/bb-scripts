import { allocateRam } from "@/lib/System";

export async function main(ns: NS) {
  const server = await allocateRam(ns, { ram: 5 }, ns => ns.getServer('foodnstuff'));
  console.log(server);
}