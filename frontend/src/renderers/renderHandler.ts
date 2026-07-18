import type { WasmFrames, WasmNode } from "../../build/pkg/kaleidoscope";
import {
  currentRenderer,
  rendererSelectDropdownOptions,
  setOptionOnClicks,
  setSelectedOption,
} from "../elements";
import { SVGRenderer } from "./svg/svg";

export interface Renderer {
  setup(): void;
  renderNode(node: WasmNode): void;
  renderFrames(node: WasmFrames): void;
}

export class RenderHandler {
  static renderer: Renderer;
  static isInit: boolean = false;
  static registeredRenderers: Map<string, new () => Renderer> = new Map();

  static Init(renderer: Renderer = new SVGRenderer()) {
    if (renderer instanceof SVGRenderer) {
      RenderHandler.Register(SVGRenderer, "Tree")
      RenderHandler.setRenderer(renderer, "Tree");
    } else {
      RenderHandler.setRenderer(renderer);
    }

    RenderHandler.renderer.setup();
  }

  static Register(renderer: new () => Renderer, name: string) {
    console.log(Array.from(this.registeredRenderers.keys()), name)

    if (Array.from(this.registeredRenderers.keys()).indexOf(name) != -1) {
      this.registeredRenderers.set(name, renderer);
      return;
    }

    this.registeredRenderers.set(name, renderer);
    rendererSelectDropdownOptions.innerHTML += `
<div class="renderer-selection-option" renderer-name="${name}">
  <div class="renderer-name">${name}</div>
  <div class="selected-outer-circle">
    <div class="selected-inner-circle"></div>
  </div>
</div>
    `;
    setOptionOnClicks();
  }

  static AssertInit() {
    if (!RenderHandler.isInit)
      throw new Error("Attempt to call to RenderHandler beore Init()");
  }

  static setRenderer(
    renderer: Renderer,
    name: string = renderer.constructor.name,
  ) {
    currentRenderer.innerText = name;
    setSelectedOption(name);

    this.renderer = renderer;
    this.renderer.setup();
  }

  static renderNode(node: WasmNode): void {
    RenderHandler.renderNode(node);
  }

  static renderFrames(frames: WasmFrames): void {
    RenderHandler.renderFrames(frames);
  }

  static setRendererByName(name: string) {
    let renderer;
    if (!(renderer = RenderHandler.registeredRenderers.get(name))) {
      let availableRenderers = Array.from(
        RenderHandler.registeredRenderers.keys(),
      );
      console.log(
        `could not find renderer: ${name}. Available renderers: ${availableRenderers}`,
      );
      return;
    }

    this.setRenderer(new renderer(), name);
  }
}
