import "../styles/main.scss";
import "mathlive";

import init from "../build/pkg/kaleidoscope.js";
import { MathFieldElement } from "./elements";

async function main() {
  await init();
  new MathFieldElement();
}

main();
