import {
  Create,
  MathTex,
  Player,
  Scene,
  Animation,
  type AnimationOptions,
  Transform,
} from "manim-web";
import type { WasmNode } from "../../../build/pkg/kaleidoscope";
import type { RendererPlaybackState } from "../../renderers/svg/svg";

export class ManimRenderer {
  static hasInit: boolean = false;
  static container: HTMLDivElement;
  static scene: Scene;
  static player: Player;
  static previousText: MathTex | null = null;

  static AssertInit() {
    if (!this.hasInit)
      throw new Error(
        "Attempt to call to ManimRender before ManimRender.Init()",
      );
  }

  constructor() {
    this.setup();
  }

  static Init() {
    if (ManimRenderer.hasInit) return;

    ManimRenderer.container = document.querySelector("#render-area")!;
    const rect = ManimRenderer.container.getBoundingClientRect();
    // ManimRenderer.scene = new Scene(ManimRenderer.container, {
    //   width: ManimRenderer.container.clientWidth,
    //   height: ManimRenderer.container.clientHeight,
    //   backgroundOpacity: 0,
    // });

    ManimRenderer.player = new Player(ManimRenderer.container, {
      width: rect.width,
      height: rect.height,
      backgroundOpacity: 1,
      backgroundColor: "#000000",
    });
    ManimRenderer.hasInit = true;
  }

  setup(): void {
    ManimRenderer.Init();
  }

  async renderRoot(node: WasmNode, wait: number = 1): Promise<void> {
    ManimRenderer.AssertInit();

    const nodeText = node.toString();

    await ManimRenderer.player.sequence(async (scene) => {
      const text = new MathTex({
        latex: nodeText,
      });

      console.log(text);
      if (ManimRenderer.previousText !== null) {
        await scene.play(new Transform(ManimRenderer.previousText, text));
      } else {
        await scene.play(new Create(text));
      }

      ManimRenderer.previousText = text;
      await scene.wait(wait);
    });

    ManimRenderer.player.play();
  }

  renderFrames(nodes: WasmNode[]): void {
    ManimRenderer.AssertInit();

    ManimRenderer.player.sequence(async (scene) => {
      for (const node of nodes) {
        const nodeText = node.toString();

        const text = new MathTex({
          latex: nodeText,
        });

        if (ManimRenderer.previousText !== null) {
          await scene.play(new Transform(ManimRenderer.previousText, text));
        } else {
          await scene.play(new Create(text));
        }

        ManimRenderer.previousText = text;
        await scene.wait(1);
      }
    });

    ManimRenderer.player.play();
    ManimRenderer.previousText = null;
  }

  stepFrame(): void {
    throw new Error("Method not implemented.");
  }

  resize(): void {
    ManimRenderer.Init();
  }

  reset(): void {
    this.setup();
  }

  state(): RendererPlaybackState {
    throw new Error("Method not implemented.");
  }

  togglePlayback(): void {
    throw new Error("Method not implemented.");
  }
}
