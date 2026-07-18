import { MathfieldElement } from "mathlive";
import { LambdaHandler } from "./handler";
import { RenderHandler } from "./renderers/renderHandler";

export const reduceButtonEl =
  document.querySelector<HTMLDivElement>("#reduce-button")!;

reduceButtonEl.addEventListener("click", () => {
  LambdaHandler.Interpret();
});

export const resetViewportButton = document.querySelector<HTMLDivElement>("#reset-viewport-button")
resetViewportButton?.addEventListener("click", () => {
  RenderHandler.Reset()
})

export const mainEl = document.querySelector<HTMLTemplateElement>("main")!;
export const addNewMathInputEl = document.querySelector<HTMLTemplateElement>(
  "#add-new-math-input",
)!;
export const mathFieldsContainer =
  document.querySelector<HTMLDivElement>("#math-inputs-list")!;

addNewMathInputEl.addEventListener("click", () => {
  new MathFieldElement();
});

let dropdownMenuShowing = false;

export const renderSelectEl =
  document.querySelector<HTMLDivElement>("#renderer-select")!;
export const currentRenderer =
  document.querySelector<HTMLDivElement>("#current-renderer")!;

export const rendererSelectDropdownOptions =
  document.querySelector<HTMLDivElement>("#render-select-dropdown-options")!;

export const renderSelectDisplay = document.querySelector<HTMLDivElement>(
  ".render-select-display",
)!;

export let rendererSelectionOptions = document.querySelectorAll<HTMLDivElement>(
  ".renderer-selection-option",
)!;

export function setSelectedOption(name: string) {
  rendererSelectionOptions = document.querySelectorAll<HTMLDivElement>(
    ".renderer-selection-option",
  )!;
  console.log(Array.from(rendererSelectionOptions));
  for (const opt of rendererSelectionOptions) {
    if (opt.getAttribute("renderer-name") == name) {
      opt.setAttribute("selected", "true");
      continue;
    }
    opt.setAttribute("selected", "false");
  }
}

export function setOptionOnClicks() {
  rendererSelectionOptions = document.querySelectorAll<HTMLDivElement>(
    ".renderer-selection-option",
  )!;
  for (const opt of rendererSelectionOptions) {
    opt.addEventListener("click", () => {
      let name;
      if (!(name = opt.getAttribute("renderer-name"))) {
        throw new Error(
          "Attempt to set a renderer without settings renderer-name in element!",
        );
      }

      RenderHandler.setRendererByName(name);
    });
  }
}

function setDropdownShowing(state: boolean = !dropdownMenuShowing) {
  if (dropdownMenuShowing) {
    rendererSelectDropdownOptions.classList.add("display-none");
    renderSelectDisplay.classList.remove("dropdown-open");
  } else {
    rendererSelectDropdownOptions.classList.remove("display-none");
    renderSelectDisplay.classList.add("dropdown-open");
  }

  dropdownMenuShowing = state;
}

renderSelectEl.addEventListener("click", () => {
  setDropdownShowing();
});

let lastId = -1;

export class MathFieldElement {
  id: number;
  mf: MathfieldElement;
  el: HTMLDivElement;
  closeButton: HTMLDivElement;

  constructor() {
    const fragment = document
      .querySelector<HTMLTemplateElement>("#math-input-template")!
      .content.cloneNode(true) as DocumentFragment;

    this.el = fragment.querySelector<HTMLDivElement>(".math-input-container")!;
    this.id = ++lastId;
    this.el.id = `math-input-${this.id}`;

    this.mf = new MathfieldElement();

    //TODO: Remove this
    this.mf.value = "abcd";

    this.el.appendChild(this.mf);

    this.closeButton = this.el.querySelector(".math-field-close")!;
    this.closeButton.id = `math-close-${this.id}`;

    this.closeButton.addEventListener("click", () => {
      this.el.remove();
      if (mathFieldsContainer.children.length == 1) {
        new MathFieldElement();
      }
    });

    mathFieldsContainer.insertBefore(
      this.el,
      mathFieldsContainer.lastElementChild,
    );

    this.mf.menuItems = [];
    this.mf.addEventListener("click", () => this.mf.focus());
    this.mf.addEventListener("keydown", (ev: KeyboardEvent) => {
      if (ev.code === "Enter") {
        LambdaHandler.ParseExpression(this.id, this.mf.value);
      }
    });

    this.mf.macros = {
      ...this.mf.macros,
      l: "λ",
    };
  }
}
