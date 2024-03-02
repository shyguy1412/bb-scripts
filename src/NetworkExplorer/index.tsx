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

    const scan: (node: string) => Tree = (node) => ns.scan(node).slice(+(node != 'home')).reduce((prev, cur) => (prev ??= {}, prev[cur] = scan(cur), prev), undefined as Tree | undefined)!;
    const servers = ns.scan('home').filter(s => !ns.getPurchasedServers().includes(s));
    const networkTree = servers.reduce((a, b) => (a.home[b] = scan(b), a), { home: {} } as Tree);
    const getDepth = (obj: Tree): number => {
      const depth = Math.max(...Object.values(obj).map(v => v ? getDepth(v) : 0)) + 1;
      return depth;
    };
    const depth = getDepth(networkTree);
    console.clear();
    const constructTreeSVG = (node: Tree, level = 0, parent?: { x: number, y: number; col: { x: number, w: number, i: number, }; }) => {
      const children = Object.keys(node);//.toSorted((a, b) => getDepth(node[a] ?? {}) - getDepth(node[b] ?? {})); //we need length later, so no for ... in
      const nextLevelChildren = children.filter((c) => node[c]);
      const nextLevelChildrenDepth = nextLevelChildren.map(c => getDepth(node[c]));
      const nextLevelChildrenDepthSum = nextLevelChildrenDepth.reduce((a, b) => a + b, 0);
      let curCol = 0;
      if (parent) {
        svg
          .rect(parent.col.x + (svg.width * 0.05), (((svg.height * 0.95) / depth) * level + (svg.height * 0.05)), parent.col.w, 10).stroke("var(--primary)");
      }
      for (let i = 0; i < children.length; i++) { //we also need index, so no for ... of
        const child = children[i];
        const leaf = node[child];


        const availableSpace =
          parent ?
            parent.col.w / children.length :
            (svg.width * 0.9) / children.length;

        const x =
          (availableSpace * i) //position along available space
          + (svg.width * 0.05) //center margins
          + (availableSpace / 2)//center node
          + (parent?.col.x ?? 0);//move to right column

        const y = (((svg.height * 0.95) / depth) * level + (svg.height * 0.05)); //take up 95% of v space and mode down acording to current depth/level

        svg
          .circle(
            x,
            (((svg.height * 0.95) / depth) * level + (svg.height * 0.05)),
            8)
          .id(child);


        if (leaf) {
          const totalColWidth = (availableSpace * children.length);
          const fractionColUnit = totalColWidth / nextLevelChildrenDepthSum;
          const fractionalColWidth = fractionColUnit * nextLevelChildrenDepth[curCol];

          const col = {
            x: nextLevelChildrenDepth.slice(0, curCol).reduce((a, b) => a + (fractionColUnit * b), 0) + (parent?.col.x ?? 0),
            w: fractionalColWidth,
            i: curCol
          };
          constructTreeSVG(leaf, level + 1, { x, y, col });
          curCol++;
        }

        if (parent) {
          svg
            .line(parent.x, parent.y, x, y).stroke('var(--primary)');
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