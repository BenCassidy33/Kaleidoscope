import { renderArea } from "../elements";

const NS_URL: string = "http://www.w3.org/2000/svg";

export class SVGCircle {
  cx: number;
  cy: number;
  r: number;
  fill: string;
  stroke: string;
  strokeWidth: number;

  constructor(
    cx: number,
    cy: number,
    r: number,
    fill: string,
    stroke: string,
    strokeWidth: number,
  ) {
    this.cx = cx;
    this.cy = cy;
    this.r = r;
    this.fill = fill;
    this.stroke = stroke;
    this.strokeWidth = strokeWidth;
  }

  toElement(): Element {
    const circle = document.createElementNS(NS_URL, "circle");

    let { x, y } = SVGNode.normalize_position(this.cx, this.cy);

    circle.setAttribute("cx", `${x}`);
    circle.setAttribute("cy", `${y}`);
    circle.setAttribute("r", `${this.r}`);
    circle.setAttribute("fill", this.fill);
    circle.setAttribute("stroke", this.stroke);
    circle.setAttribute("stroke-width", `${this.strokeWidth}`);

    return circle;
  }
}

export class SVGNode {
  static normalize_position(x: number, y: number): { x: number; y: number } {
    return {
      x: x + renderArea.clientWidth / 2,
      y: y + renderArea.clientHeight / 2,
    };
  }

  static setAttributes(e: Element, attributes: Object) {
    for (const key of Object.keys(attributes)) {
      // @ts-ignore
      e.setAttribute(key, `${attributes[key]}`);
    }
  }

  constructor() {
    const svg = document.createElementNS(NS_URL, "svg");

    svg.setAttribute("width", `${renderArea?.clientWidth}`);
    svg.setAttribute("height", `${renderArea?.clientHeight}`);
    svg.setAttribute(
      "viewBox",
      `0 0 ${renderArea?.clientWidth} ${renderArea?.clientHeight}`,
    );

    const circle = new SVGCircle(
      0,
      0,
      100,
      "lightblue",
      "black",
      4,
    );

    let circleE = circle.toElement();

    svg.appendChild(circleE);
    renderArea?.appendChild(svg);

    circleE.animate([{ r: 0 }, { r: circle.r }], {
      duration: 500,
      easing: "cubic-bezier(0,1.5,.31,1.19)",
      fill: "forwards",
    });
  }
}
