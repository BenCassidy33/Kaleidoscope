export const mainElement = document.querySelector<HTMLTemplateElement>("main")!;

let lastId = -1;

export class MathTemplate {
  el: HTMLDivElement;

  constructor() {
    console.log(
      document.querySelector("#math-input-template"),

      document
        .querySelector<HTMLTemplateElement>("#math-input-template")!
        .content.cloneNode(true) as HTMLDivElement,
    );

    this.el = document
      .querySelector<HTMLTemplateElement>("#math-input-template")!
      .content.cloneNode(true) as HTMLDivElement;

    this.el.id = `math-input-${lastId++}`;

    document
      .querySelector<HTMLDivElement>("#math-inputs-container")
      ?.appendChild(this.el);
  }
}
