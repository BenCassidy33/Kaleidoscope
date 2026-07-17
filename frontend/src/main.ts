import "../styles/main.scss";
import "mathlive";

import init from "../build/pkg/kaleidoscope.js";
import { MathFieldElement } from "./elements";
import { SVGNode } from "./utils/svgTree.js";

async function main() {
  await init();
  new MathFieldElement();

  new SVGNode();
}

main();
