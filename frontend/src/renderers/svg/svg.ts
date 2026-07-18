import type { WasmFrames, WasmNode } from "../../../build/pkg/kaleidoscope";
import { type Renderer } from "../renderHandler";
import { SVGNode } from "./node";

export const SVG_NS_URL: string = "http://www.w3.org/2000/svg";
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
