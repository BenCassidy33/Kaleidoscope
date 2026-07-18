import type { WasmFrames, WasmNode } from "../../../build/pkg/kaleidoscope";
import { TODO } from "../../utils";
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
    console.log(frames[1]?.toJson(true));
    for (const wasm_node of frames) {
      SVGRenderer.nodes = [];
      SVGRenderer.viewport.querySelectorAll("g").forEach((g) => g.remove());
      this.renderNode(wasm_node);
    }

    TODO("Render frames as animation!");
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

  static AddNode(node: SVGNode) {
    SVGRenderer.AssertInit();
    SVGRenderer.nodes.push(node);
    // SVGRenderer.viewport.appendChild(node.toElement());
  }

  static Render() {
    SVGRenderer.AssertInit();

    SVGRenderer.renderContainerEl.innerHTML = "";
    SVGRenderer.renderContainerEl.appendChild(SVGRenderer.viewport);

    for (const node of SVGRenderer.nodes) {
      node.drawConnections(SVGRenderer.viewport, {
        stroke: "white",
        strokeWidth: 1,
      });
    }

    for (const node of SVGRenderer.nodes) {
      SVGRenderer.viewport.appendChild(node.toElement());
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
