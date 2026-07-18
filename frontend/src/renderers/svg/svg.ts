import type { WasmNode } from "../../../build/pkg/kaleidoscope";
import { Point, TODO, Utils, ViewBox } from "../../utils";
import { type Renderer } from "../renderHandler";
import { SVGNode } from "./node";

export const SVG_NS_URL: string = "http://www.w3.org/2000/svg";

export class SVGRenderer implements Renderer {
  static nodes: SVGNode[] = [];
  static renderContainerEl: HTMLDivElement;
  static viewport: SVGElement;

  private static m_hasInit: boolean = false;

  private static m_clientStartX: number;
  private static m_clientStartY: number;
  private static m_viewportStartX: number;
  private static m_viewportStartY: number;

  private static isHoldingNode: boolean = false;

  private static shouldNodeAnimationsPlay = true;
  private static nodeAnimationId: number = -1;

  setup() {
    SVGRenderer.Init();
  }

  renderNode(node: WasmNode): void {
    SVGRenderer.nodes = [];

    const root = SVGNode.FromWasmNode(node);
    if (!root) {
      return;
    }

    SVGRenderer.nodes = SVGRenderer.FlattenNodes(root);
    SVGRenderer.Render();
  }

  reset() {
    SVGRenderer.ResetViewport();
    SVGRenderer.RedrawBackground();
  }

  static FlattenNodes(node: SVGNode | undefined): SVGNode[] {
    if (!node) {
      return [];
    }

    return [
      node,
      ...SVGRenderer.FlattenNodes(node.left),
      ...SVGRenderer.FlattenNodes(node.right),
    ];
  }

  renderFrames(frames: WasmNode[]) {
    for (const wasm_node of frames) {
      SVGRenderer.nodes = [];
      SVGRenderer.viewport.querySelectorAll("g").forEach((g) => g.remove());
      this.renderNode(wasm_node);
    }
  }

  resize() {
    SVGRenderer.ResetViewport();
    SVGRenderer.RedrawBackground();
  }

  static Init() {
    if (this.m_hasInit) {
      console.warn("Warning! Call to Init() on SVGRenderer more than once!");
      return;
    }

    SVGRenderer.renderContainerEl =
      document.querySelector<HTMLDivElement>("#render-area")!;

    SVGRenderer.viewport = document.createElementNS(
      SVG_NS_URL,
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

    SVGRenderer.CreateBackground();
  }

  static CreateBackground() {
    let defs = document.createElementNS(SVG_NS_URL, "defs");
    let pattern = document.createElementNS(SVG_NS_URL, "pattern");

    const patternWidth = 50;
    SVGNode.setAttributes(pattern, {
      id: "tile",
      width: patternWidth,
      height: patternWidth,
      patternUnits: "userSpaceOnUse",
    });

    const circle = document.createElementNS(SVG_NS_URL, "circle");
    SVGNode.setAttributes(circle, {
      cx: patternWidth / 2,
      cy: patternWidth / 2,
      r: 1.5,
      fill: "gray",
    });

    pattern.appendChild(circle);
    defs.appendChild(pattern);
    SVGRenderer.viewport.appendChild(defs);

    const backgroundSize = 200;

    let backgroundRect = document.createElementNS(SVG_NS_URL, "rect");
    SVGNode.setAttributes(backgroundRect, {
      id: "background-rect",
      x: `-${backgroundSize / 4}%`,
      y: `-${backgroundSize / 4}%`,
      width: `${backgroundSize}%`,
      height: `${backgroundSize}%`,
      fill: "url(#tile)",
    });

    SVGRenderer.viewport.appendChild(backgroundRect);
  }

  static RedrawBackground() {
    const pattern = document.querySelector("#tile")!;
    const patternRect = document.querySelector("#background-rect")!;

    const [vx, vy, _] = SVGRenderer.viewport
      .getAttribute("viewBox")!
      .split(" ");
    const [x, y] = [parseFloat(vx!), parseFloat(vy!)];

    SVGNode.setAttributes(pattern, {
      width: 0,
    });

    SVGNode.setAttributes(pattern, {
      width: 50,
    });

    SVGNode.setAttributes(patternRect, {
      x: x,
      y: y,
    });
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
    const newWidth = SVGRenderer.renderContainerEl.clientWidth;
    const newHeight = SVGRenderer.renderContainerEl.clientHeight;

    SVGNode.setAttributes(SVGRenderer.viewport, {
      width: newWidth,
      height: newHeight,
      viewBox: `0 0 ${newWidth} ${newHeight}`,
    });
  }

  private static HandleMouseMove(ev: MouseEvent) {
    SVGRenderer.RedrawBackground();

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

    if (SVGRenderer.isHoldingNode) {
    } else {
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
  }

  static HandleHeldNodeMove(node: SVGNode, el: Element) {
    node.shouldNodeAnimationPlay = false;

    const circleNode = node.toElement().querySelector("circle")!;
    const text = node.toElement().querySelector("text")!;

    const startX = node.cx;
    const startY = node.cy;
    let targetX = node.cx;
    let targetY = node.cy;

    const onMove = (ev: MouseEvent) => {
      const rect = SVGRenderer.viewport.getBoundingClientRect();
      const viewbox = ViewBox.Get(this.viewport as Element);

      targetX = ev.clientX - rect.left + viewbox.x;
      targetY = ev.clientY - rect.top + viewbox.y;
    };

    window.addEventListener("mousemove", onMove);

    let isRunning = true;

    const lag = 0.12;

    const step = () => {
      if (!isRunning) return;

      node.cx += (targetX - node.cx) * lag;
      node.cy += (targetY - node.cy) * lag;

      SVGNode.setAttributes(circleNode, { cx: node.cx, cy: node.cy });
      SVGNode.setAttributes(text, { x: node.cx, y: node.cy });

      SVGRenderer.RenderConnections();

      requestAnimationFrame(step);
    };

    requestAnimationFrame(step);

    window.addEventListener("mouseup", () => {
      isRunning = false;
      window.removeEventListener("mousemove", onMove);
      SVGRenderer.HandleNodeReleased(node, el);
    })
  }

  static HandleHeldNodeMovedInner(ev: MouseEvent, node: SVGNode) {}

  static HandleNodeReleased(node: SVGNode, _: Element) {
    node.shouldNodeAnimationPlay = true;
    this.isHoldingNode = false;
    node.startAnimation();
  }

  static AddNode(node: SVGNode) {
    SVGRenderer.AssertInit();
    SVGRenderer.nodes.push(node);
  }

  /// TODO: redo this so that the nodes get updated in place instead of removed and readded
  static RenderConnections() {
    document.querySelector("#connection-line-group")?.remove();

    const g = document.createElementNS(SVG_NS_URL, "g") as SVGGElement;
    g.id = "connection-line-group";

    const backgroundRect = document.querySelector("#background-rect");

    if (backgroundRect && backgroundRect.nextSibling) {
      SVGRenderer.viewport.insertBefore(g, backgroundRect.nextSibling);
    } else {
      SVGRenderer.viewport.appendChild(g);
    }

    for (const node of SVGRenderer.nodes) {
      node.drawConnections(g, {
        stroke: "white",
        strokeWidth: 1,
      });
    }
  }

  static RenderNodes() {
    for (const node of SVGRenderer.nodes) {
      const el = node.toElement();
      SVGRenderer.viewport.appendChild(el);

      el.addEventListener("mousedown", () => {
        node.shouldNodeAnimationPlay = false;
        this.isHoldingNode = true;

        SVGRenderer.HandleHeldNodeMove(node, el);
      });
    }
  }

  static Render(shouldAnimate: boolean = true) {
    SVGRenderer.AssertInit();
    SVGRenderer.renderContainerEl.innerHTML = "";
    SVGRenderer.renderContainerEl.appendChild(SVGRenderer.viewport);
    SVGRenderer.RenderConnections();
    SVGRenderer.RenderNodes();

    if (shouldAnimate) SVGRenderer.StartNodeAnimations();
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

  static StartNodeAnimations() {
    for (const node of SVGRenderer.nodes) {
      node.startAnimation();
    }
  }
}
