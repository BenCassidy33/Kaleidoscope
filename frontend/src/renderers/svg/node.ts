import { type WasmNode } from "../../../build/pkg/kaleidoscope";
import { SVG_NS_URL, SVGRenderer } from "./svg";

const X_NODE_SEPERATION: number = 80;
const Y_NODE_SEPERATION: number = 80;

type SVGCircleAttrs = {
  id?: string;
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

const CASE_SENSITIVE_ATTRS = new Set([
  "viewBox",
  "preserveAspectRatio",
  "patternUnits",
]);

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
      r: 25,
      fill: "white",
      stroke: "black",
      strokeWidth: "4",
    },
    inner?: SVGNode | SVGElement,
    left?: SVGNode,
    right?: SVGNode,
    normalize: boolean = true,
  ) {
    this.attributes = attributes;
    this.left = left;
    this.right = right;
    if (inner) this.setInner(inner);

    if (normalize) {
      let { x, y } = SVGNode.NormalizePosition(attributes.cx!, attributes.cy!);
      this.cx = x;
      this.cy = y;
    } else {
      this.cx = attributes.cx!;
      this.cy = attributes.cy!;
    }

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

    circle.classList.add("svgNode");
    return circle;
  }

  get element() {
    this.el = this.toElement();
    return this.el;
  }

  setInner(inner: SVGNode | SVGElement, isNormalized: boolean = true) {
    if (inner instanceof SVGElement) {
      if (isNormalized) {
        SVGNode.setAttributes(inner, {
          x: this.cx,
          y: this.cy,
        });
      } else {
        const { x, y } = SVGNode.NormalizePosition(this.cx, this.cy);
        SVGNode.setAttributes(inner, {
          x: x,
          y: y,
        });
      }
      this.inner = inner;
    } else {
      inner.cx = this.cx;
      inner.cy = this.cy;

      this.inner = inner;
    }
  }

  drawConnections(viewport: SVGElement, attributes: Object) {
    if (this.left) {
      let root = SVGNode.NormalizePosition(this.cx, this.cy);
      let left = SVGNode.NormalizePosition(this.left.cx, this.left.cy);
      const line = document.createElementNS(
        SVG_NS_URL,
        "line",
      ) as SVGLineElement;
      line.classList.add("connecting-line");

      SVGNode.setAttributes(line, {
        x1: root.x,
        x2: left.x,
        y1: root.y,
        y2: left.y,
        ...attributes,
      });

      viewport.appendChild(line);
    }

    if (this.right) {
      let root = SVGNode.NormalizePosition(this.cx, this.cy);
      let right = SVGNode.NormalizePosition(this.right.cx, this.right.cy);
      const line = document.createElementNS(
        SVG_NS_URL,
        "line",
      ) as SVGLineElement;
      line.classList.add("connecting-line");

      SVGNode.setAttributes(line, {
        x1: root.x,
        x2: right.x,
        y1: root.y,
        y2: right.y,
        ...attributes,
      });

      viewport.appendChild(line);
    }
  }

  static CreateTextElement(innerText: string): SVGTextElement {
    const text = document.createElementNS(SVG_NS_URL, "text") as SVGTextElement;
    text.classList.add("node-text");
    text.textContent = innerText;

    SVGNode.setAttributes(text, {
      textAnchor: "middle",
      dominantBaseline: "middle",
    });

    return text;
  }

  static FromWasmNode(
    node: WasmNode,
    depth: number = 1,
    parent?: SVGNode,
    isLeft: boolean = false,
  ): SVGNode | void {
    const x = isLeft ? -1 : 1;
    const pos = (parent ? parent.cx : 0) + x * X_NODE_SEPERATION;
    const id = Math.random() * 9_152_052;

    let svg_node = new SVGNode(
      {
        id: `${id}`,
        cx: pos,
        cy: Y_NODE_SEPERATION * (depth * 0.8),
        r: 30,
        fill: "white",
        stroke: "none",
        strokeWidth: "0",
      },
      undefined,
      undefined,
      undefined,
      false,
    );

    const node_inner = node.inner();

    if (node_inner.is_variable()) {
      const variable = node_inner.variable();

      svg_node.setInner(SVGNode.CreateTextElement(variable.ident), false);
      return svg_node;
    }

    if (node_inner.is_application()) {
      const application = node_inner.application();
      const text = SVGNode.CreateTextElement(application.toString());
      svg_node.setInner(text, false);

      const left = SVGNode.FromWasmNode(
        application.left,
        depth + 1,
        svg_node,
        true,
      );

      const right = SVGNode.FromWasmNode(
        application.right,
        depth + 1,
        svg_node,
        false,
      );

      if (left) {
        svg_node.left = left;
      }

      if (right) {
        svg_node.right = right;
      }

      return svg_node;
    }

    const abstraction = node_inner.abstraction();
    svg_node.setInner(SVGNode.CreateTextElement(abstraction.toString()), false);

    const left = SVGNode.FromWasmNode(
      abstraction.bound,
      depth + 1,
      svg_node,
      true,
    );

    const right = SVGNode.FromWasmNode(
      abstraction.body,
      depth + 1,
      svg_node,
      false,
    );

    if (left) {
      svg_node.left = left;
    }

    if (right) {
      svg_node.right = right;
    }

    return svg_node;
  }
}
