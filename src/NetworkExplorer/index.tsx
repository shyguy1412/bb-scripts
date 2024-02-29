import React, { useContext, useEffect, useState } from "react";
import { GeckoSVG } from 'geckosvg';
import { NetscriptContext } from "@/lib/Context";
import { Tree } from "@/lib/Types";

export function NetworkExplorer() {
  "use scan";
  "use getPurchasedServers";
  const [svg] = useState(GeckoSVG.create()); //as state to preserve instance

  const ns = useContext(NetscriptContext);

  useEffect(() => {
    svg.style.display = 'block';
    svg.style.height = '100%';
    svg.style.width = '100%';
    svg.style.overflow = 'hidden';

    const resizeSvg = () => {
      const { width, height } = svg.getBoundingClientRect();
      svg.width = Math.floor(width);
      svg.height = Math.floor(height);
    };
    new ResizeObserver(resizeSvg).observe(svg);
    resizeSvg();

    const scan: (node: string) => Tree = (node) => ns.scan(node).slice(+(node != 'home')).reduce((prev, cur) => (prev ??= {}, prev[cur] = scan(cur), prev), undefined as Tree);
    const servers = ns.scan('home').filter(s => !ns.getPurchasedServers().includes(s));
    const networkTree = servers.reduce((a, b) => (a.home[b] = scan(b), a), { home: {} });
    const getDepth = (obj: Tree) => {
      const depth = Math.max(...Object.values(obj).map(v => v ? getDepth(v) : 0)) + 1;
      return depth;
    };
    const depth = getDepth(networkTree);

    const constructTreeSVG = (node: Tree, level = 0, parent?: { x: number, y: number; }) => {
      const children = Object.keys(node); //we need length later, so no for ... in
      const nextLevelChildren = children.reduce((prev, cur) => prev + (+!!node[cur]), 0);
      console.log(nextLevelChildren, Object.keys(node)[0]);
      for (let i = 0; i < children.length; i++) { //we also need index, so no for ... of
        const child = children[i];
        const leaf = node[child];

        const x = (
          ((svg.width * 0.8) / children.length) //space one node should take up
          * i) //position along available space
          + (svg.width * 0.1) //center margins
          + (((svg.width * 0.8) / children.length) / 2);//center node

        const y = (((svg.height * 0.95) / depth) * level + (svg.height * 0.05)); //take up 95% of v space and mode down acording to current depth/level

        svg
          .circle(
            x,
            (((svg.height * 0.95) / depth) * level + (svg.height * 0.05)),
            10)
          .id(child);

        if (parent) {
          svg
            .line(parent.x, parent.y, x, y).stroke('var(--primary)');
        }

        if (leaf) {
          constructTreeSVG(leaf, level + 1, { x, y });
        }
      }

      // console.log({ depth, length: children.length, node: Object.keys(node)[0] });

      // svg
      //   .rect(
      //     (((svg.width * 0.8) / children.length) * 0) + (svg.width * 0.1),
      //     (((svg.height * 0.8) / depth) * (level + 1)) + (svg.height * 0.1) - 10,
      //     (svg.width * 0.8) / children.length,
      //     (svg.height * 0.8) / depth - 10
      //   );
    };

    constructTreeSVG(networkTree);
    console.log(networkTree);

  }, [svg]);

  return <div style={{ height: '100%' }} ref={ref => ref?.append(svg)}></div>;
}