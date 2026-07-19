import { type WasmNode } from "../../../build/pkg/kaleidoscope";
import { Utils } from "../../utils";
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
  el: Element | undefined;

  cx: number;
  cy: number;

  shouldNodeAnimationPlay: boolean = true;
  parent: SVGNode | undefined;

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
  ) {
    this.attributes = attributes;
    this.left = left;
    this.right = right;
    if (inner) this.setInner(inner);

    this.cx = attributes.cx!;
    this.cy = attributes.cy!;
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
    if (this.el) return this.el;

    const circle = document.createElementNS(SVG_NS_URL, "circle");
    const [x, y] = Utils.NormalizeClientCoordsToViewport(
      this.attributes.cx!,
      this.attributes.cy!,
      SVGRenderer.viewport,
    );

    SVGNode.setAttributes(circle, {
      ...this.attributes,
      cx: x,
      cy: y,
    });

    if (this.inner !== undefined) {
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

    this.el = undefined as any;
    this.el = this.toElement();
  }

  drawConnections(group: SVGElement, attributes: Object) {
    if (this.left) {
      const line = document.createElementNS(
        SVG_NS_URL,
        "line",
      ) as SVGLineElement;
      line.classList.add("connecting-line");

      SVGNode.setAttributes(line, {
        x1: this.cx,
        x2: this.left.cx,
        y1: this.cy,
        y2: this.left.cy,
        ...attributes,
      });

      group.appendChild(line);
    }

    if (this.right) {
      const line = document.createElementNS(
        SVG_NS_URL,
        "line",
      ) as SVGLineElement;
      line.classList.add("connecting-line");

      SVGNode.setAttributes(line, {
        x1: this.cx,
        x2: this.right.cx,
        y1: this.cy,
        y2: this.right.cy,
        ...attributes,
      });

      group.appendChild(line);
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
    isRoot: boolean = false,
  ): SVGNode | void {
    const x = isLeft ? -1 : 1;
    const posX = isRoot ? 0 : (parent ? Utils.UnnormalizeCoordsFromViewport(parent.attributes.cx!, 0, SVGRenderer.viewport)[0]! : 0) + (x * X_NODE_SEPERATION);
    const posY = isRoot ? 0 : Y_NODE_SEPERATION * (depth * 0.8);

    const [px, py] = Utils.NormalizeCoordsToViewport(posX, posY, SVGRenderer.viewport);

    const id = Math.round(Math.random() * 9_152_052);

    let svg_node = new SVGNode(
      {
        id: `${id}`,
        cx: px,
        cy: py,
        r: 30,
        fill: "white",
        stroke: "none",
        strokeWidth: "0",
      },
      undefined,
      undefined,
      undefined,
    );

    svg_node.parent = parent;

    const node_inner = node.inner();
    if (node_inner.is_variable()) {
      const variable = node_inner.variable();

      svg_node.setInner(SVGNode.CreateTextElement(variable.ident));
      return svg_node;
    }

    if (node_inner.is_application()) {
      const application = node_inner.application();
      const text = SVGNode.CreateTextElement(application.toString());
      svg_node.setInner(text);

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
    svg_node.setInner(SVGNode.CreateTextElement(abstraction.toString()));

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

  startAnimation() {
    let startTime: number | null = null;

    const startX = this.cx;
    const startY = this.cy;

    const groupEl = this.toElement();
    let circleEl;

    if (groupEl.tagName == "g") {
      circleEl = groupEl.querySelector("circle")!;
    } else {
      circleEl = groupEl;
    }

    const textEl = this.el!.querySelector("text");

    const speed = 0.00085;
    const size = 10;
    const rotationSin = Math.random() - 1;
    const rotationCos = Math.random() - 1;

    const step = (timestamp: number = Math.random() * 100) => {
      if (startTime === null) startTime = timestamp;

      const elapsed = timestamp - startTime;

      const cx = startX + size * Math.sin(elapsed * speed * rotationSin);
      const cy = startY + size * Math.cos(elapsed * speed * rotationCos);

      this.cx = cx;
      this.cy = cy;

      SVGNode.setAttributes(circleEl!, {
        cx: cx,
        cy: cy,
      });

      if (textEl)
        SVGNode.setAttributes(textEl, {
          x: cx,
          y: cy,
        });

      SVGRenderer.RenderConnections();
      if (this.shouldNodeAnimationPlay) requestAnimationFrame(step);
    };

    requestAnimationFrame(step);
  }
}
