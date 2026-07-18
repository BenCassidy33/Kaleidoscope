import "../styles/main.scss";
import "mathlive";

import init from "../build/pkg/kaleidoscope.js";
import { MathFieldElement } from "./elements";
import { SVGRenderer } from "./renderers/svg.js";
import { LambdaHandler } from "./handler.js";
import { RenderHandler } from "./renderers/renderHandler.js";

async function initMain() {
  await init();
  SVGRenderer.Init();
  LambdaHandler.Init();
  RenderHandler.Init();
}

async function main() {
  await initMain();

  new MathFieldElement();
}

// let text = RawSVG.create<SVGTextElement>("text", {
//   x: 0,
//   y: 0,
//   fontSize: 16,
//   fill: "black",
//   textAnchor: "middle",
//   dominantBaseline: "middle",
// });
//
// text.textContent = "hello!";
//
// const c = new SVGNode({ cx: 0, cy: 0, r: 50 });
// c.setInner(text);
//
// SVGRenderer.AddNode(c.toElement());
// SVGRenderer.Render();
main();
