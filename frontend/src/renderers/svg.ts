import type { WasmFrames } from "../../build/pkg/kaleidoscope";
import { type Renderer } from "./renderHandler";

const NS_URL: string = "http://www.w3.org/2000/svg";
window.addEventListener("resize", () => {});

export class SVGRenderer implements Renderer {
  static nodes: Element[] = [];
  static renderContainerEl: HTMLDivElement;
  static viewport: SVGElement;

  private static m_hasInit: boolean = false;

  private static m_clientStartX: number;
  private static m_clientStartY: number;
  private static m_viewportStartX: number;
  private static m_viewportStartY: number;

  setup() {
    SVGRenderer.Init()
  }

  renderNode(node: WasmNode): void {};

  renderFrames(frames: WasmFrames) {
    throw new Error("Todo!");
  }

  static Init() {
    SVGRenderer.renderContainerEl =
      document.querySelector<HTMLDivElement>("#render-area")!;

    SVGRenderer.viewport = document.createElementNS(
      NS_URL,
      "svg",
    ) as SVGElement;

    this.renderContainerEl.addEventListener(
      "mousedown",
      SVGRenderer.HandleMouseDown,
    );

    SVGNode.setAttributes(SVGRenderer.viewport, {
      width: SVGRenderer.renderContainerEl.clientWidth,
      height: SVGRenderer.renderContainerEl.clientHeight,
      viewBox: `0 0 ${SVGRenderer.renderContainerEl?.clientWidth} ${SVGRenderer.renderContainerEl?.clientHeight}`,
    });

    SVGRenderer.renderContainerEl?.appendChild(SVGRenderer.viewport);
    this.m_hasInit = true;
  }

  private static AssertInit() {
    if (!SVGRenderer.m_hasInit)
      throw new Error(
        "Attempt to call to SVGRenderer before init() has been called!",
      );
  }

  private static MapClientCoordinates(
    cx: number,
    cy: number,
  ): [number, number] {
    const rect = SVGRenderer.viewport.getBoundingClientRect();
    cx = cx - rect.left - rect.width / 2;
    cy = cy - rect.top - rect.height / 2;

    return [cx, cy];
  }

  static ResetViewport() {
    SVGNode.setAttributes(SVGRenderer.viewport, {
      viewBox: `0 0 ${SVGRenderer.renderContainerEl.clientWidth} ${SVGRenderer.renderContainerEl.clientHeight}`,
    });
  }

  private static HandleMouseMove(ev: MouseEvent) {
    const [csx, csy] = SVGRenderer.MapClientCoordinates(
      this.m_clientStartX,
      this.m_clientStartY,
    );

    const [cx, cy] = SVGRenderer.MapClientCoordinates(ev.clientX, ev.clientY);

    const cdx = csx - cx;
    const cdy = csy - cy;

    SVGNode.setAttributes(SVGRenderer.viewport, {
      viewBox: `${this.m_viewportStartX + cdx} ${this.m_viewportStartY + cdy} ${SVGRenderer.renderContainerEl.clientWidth} ${SVGRenderer.renderContainerEl.clientHeight}`,
    });
  }

  private static HandleMouseDown(ev: MouseEvent) {
    let [vpsx, vpsy, _] = SVGRenderer.viewport
      .getAttribute("viewBox")!
      .split(" ");

    this.m_viewportStartX = parseFloat(vpsx!);
    this.m_viewportStartY = parseFloat(vpsy!);

    this.m_clientStartX = ev.clientX;
    this.m_clientStartY = ev.clientY;

    SVGRenderer.renderContainerEl.addEventListener(
      "mousemove",
      SVGRenderer.HandleMouseMove,
    );

    SVGRenderer.renderContainerEl.addEventListener("mouseup", () => {
      SVGRenderer.renderContainerEl.removeEventListener(
        "mousemove",
        SVGRenderer.HandleMouseMove,
      );
    });
  }

  static AddNode(element: Element) {
    SVGRenderer.AssertInit();

    SVGRenderer.nodes.push(element);
    SVGRenderer.viewport.appendChild(element);
  }

  static Render() {
    SVGRenderer.AssertInit();

    SVGRenderer.renderContainerEl.innerHTML = "";
    SVGRenderer.renderContainerEl.appendChild(SVGRenderer.viewport);

    for (const node of SVGRenderer.nodes) {
      SVGRenderer.viewport.appendChild(node);
    }
  }

  static get SVG(): SVGElement {
    SVGRenderer.AssertInit();
    return this.SVG;
  }

  static get RenderContainer(): HTMLDivElement {
    SVGRenderer.AssertInit();
    return document.querySelector<HTMLDivElement>("#render-area")!;
  }

  static get ClientWidth(): number {
    SVGRenderer.AssertInit();
    return SVGRenderer.renderContainerEl.clientWidth;
  }

  static get ClientHeight(): number {
    SVGRenderer.AssertInit();
    return SVGRenderer.renderContainerEl.clientHeight;
  }
}

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
    const e = document.createElementNS(NS_URL, kind) as T;
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
    const circle = document.createElementNS(NS_URL, "circle");
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
      const group = document.createElementNS(NS_URL, "g");
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
