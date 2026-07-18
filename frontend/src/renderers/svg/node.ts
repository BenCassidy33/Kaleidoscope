import { SVG_NS_URL, SVGRenderer } from "./svg";

type SVGCircleAttrs = {
  cx?: number;
  cy?: number;
  r?: number;
  fill?: string;
  stroke?: string;
  strokeWidth?: string;
  textAnchor?: string;
  dominantBaseline?: string;
};

export class RawSVG {
  static create<T extends SVGElement>(kind: string, attributes: Object): T {
    // @ts-ignore
    const e = document.createElementNS(SVG_NS_URL, kind) as T;
    SVGNode.setAttributes(e, attributes);

    return e;
  }
}

const CASE_SENSITIVE_ATTRS = new Set(["viewBox", "preserveAspectRatio"]);

export class SVGNode {
  left: SVGNode | undefined;
  right: SVGNode | undefined;
  inner: SVGNode | SVGElement | undefined;

  attributes: Partial<SVGCircleAttrs>;
  el: Element;

  cx: number;
  cy: number;

  constructor(
    attributes: Partial<SVGCircleAttrs> = {
      cx: 0,
      cy: 0,
      r: 50,
      fill: "lightblue",
      stroke: "black",
      strokeWidth: "4",
    },
    inner?: SVGNode | SVGElement,
    left?: SVGNode,
    right?: SVGNode,
  ) {
    this.attributes = attributes;
    this.left = left;
    this.right = right;
    if (inner) this.setInner(inner);

    let { x, y } = SVGNode.NormalizePosition(attributes.cx!, attributes.cy!);
    this.cx = x;
    this.cy = y;

    this.el = this.toElement();
  }

  static NormalizePosition(x: number, y: number): { x: number; y: number } {
    return {
      x: x + SVGRenderer.ClientWidth / 2,
      y: y + SVGRenderer.ClientHeight / 2,
    };
  }

  static setAttributes(e: Element, attributes: Object) {
    for (const [key, value] of Object.entries(attributes)) {
      const attrName = CASE_SENSITIVE_ATTRS.has(key)
        ? key
        : key.replace(/([A-Z])/g, "-$1").toLowerCase();
      e.setAttribute(attrName, `${value}`);
    }
  }

  toElement(): Element {
    const circle = document.createElementNS(SVG_NS_URL, "circle");
    let { x, y } = SVGNode.NormalizePosition(
      //@ts-ignore
      this.attributes.cx!,
      this.attributes.cy!,
    );

    SVGNode.setAttributes(circle, {
      ...this.attributes,
      cx: x,
      cy: y,
      fill: "lightblue",
      stroke: "black",
      strokeWidth: "4",
    });

    if (this.inner) {
      const group = document.createElementNS(SVG_NS_URL, "g");
      group.appendChild(circle);

      if (this.inner instanceof SVGElement) {
        group.appendChild(this.inner);
      } else if (this.inner instanceof SVGNode) {
        group.appendChild(this.inner.toElement());
      }

      group.classList.add("svgGroup");
      return group;
    }

    circle.classList.add("svgNode")
    return circle;
  }

  get element() {
    this.el = this.toElement();
    return this.el;
  }

  setInner(inner: SVGNode | SVGElement) {
    if (inner instanceof SVGElement) {
      SVGNode.setAttributes(inner, {
        x: this.cx,
        y: this.cy,
      });

      this.inner = inner;
    } else {
      inner.cx = this.cx;
      inner.cy = this.cy;

      this.inner = inner;
    }
  }
}
