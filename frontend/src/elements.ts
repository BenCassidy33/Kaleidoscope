import { MathfieldElement } from "mathlive";


export const mainEl = document.querySelector<HTMLTemplateElement>("main")!;
export const addNewMathInputEl = document.querySelector<HTMLTemplateElement>(
  "#add-new-math-input",
)!;
export const mathFieldsContainer =
  document.querySelector<HTMLDivElement>("#math-inputs-list")!;

addNewMathInputEl.addEventListener("click", () => {
  new MathFieldElement();
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

    this.mf.macros = {
      ...this.mf.macros,
      l: "λ"
    }
  }
}
