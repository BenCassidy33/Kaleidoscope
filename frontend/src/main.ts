import "../styles/main.scss";
import "mathlive";

import init from "../build/pkg/kaleidoscope.js";
import { MathTemplate } from "./elements";

async function main() {
  await init();
  new MathTemplate();
  console.log("Hello!");
}

main();
