import type { WasmFrames, WasmNode } from "../../build/pkg/kaleidoscope";
import { SVGRenderer } from "./svg";

export interface Renderer {
  setup(): void;
  renderNode(node: WasmNode): void;
  renderFrames(node: WasmFrames): void;
}

export class RenderHandler {
  static renderer: Renderer
  static isInit: boolean = false;

  static Init(renderer: Renderer = new SVGRenderer()) {
    RenderHandler.renderer = renderer;
    RenderHandler.renderer.setup();
  }

  static AssertInit() {
    if (!RenderHandler.isInit)
      throw new Error("Attempt to call to RenderHandler beore Init()")
  }

  static setRenderer(renderer: Renderer) {
    this.renderer = renderer;
    this.renderer.setup()
  }

  static renderNode(node: WasmNode): void {
    RenderHandler.renderNode(node);
  }

  static renderFrames(frames: WasmFrames): void {
    RenderHandler.renderFrames(frames);
  }
}
