import type { MathfieldElement } from "mathlive";
import { Lambda, ParsingError, WasmAssignment, WasmNode } from "../build/pkg/kaleidoscope";
import { mathFieldsContainer } from "./elements";

export type ExpressionEntry = {
  fieldId: number;
  raw: string;
  expression: WasmNode;
};

export type AssignmentEntry = {
  fieldId: number;
  assignment: WasmAssignment;
};

export class LambdaHandler {
  private static hasInit: boolean = false;
  private static assignments: Map<number, WasmAssignment>;
  private static statements: Map<number, WasmNode>;
  private static lastStatementInsertId: number = -1;

  static Init() {
    LambdaHandler.assignments = new Map();
    LambdaHandler.statements = new Map();

    this.hasInit = true;
  }

  static AssertInit() {
    if (!this.hasInit)
      throw new Error(
        "Attempt to call to Handler before call to Handler.Init()",
      );
  }

  static ParseExpression(fieldId: number, expression: string) {
    LambdaHandler.AssertInit();

    let lambda: Lambda[];
    try {
       lambda = Lambda.parse(
        LambdaHandler.NormalizeLatex(expression),
      );
    } catch (e) {
      console.log("Error = ", (e as ParsingError).toJson(true));
      return;
    }

    for (const l of lambda) {
      if (l.kind.isAssignment()) {
        LambdaHandler.assignments.set(fieldId, l.kind.assignment!);
      } else {
        LambdaHandler.statements.set(fieldId, l.kind.statement!);
        this.lastStatementInsertId = fieldId;
      }
    }
  }

  static ReduceLastStatement() {
    if (this.lastStatementInsertId == -1) {
      const lastChild = mathFieldsContainer.children[
        mathFieldsContainer.children.length - 2
      ]!.querySelector("math-field") as MathfieldElement;

      if (!lastChild || !lastChild.value || lastChild.value === "") {
        return;
      }

      const id = lastChild.id.split("-")[2]!;
      LambdaHandler.ParseExpression(parseInt(id), lastChild.value);
    }

    let item;

    if (
      !(item = LambdaHandler.statements.get(
        LambdaHandler.lastStatementInsertId,
      ))
    ) {
      console.error(
        `Failed to get assignment with id: ${LambdaHandler.lastStatementInsertId}`,
      );
      return;
    }

    console.log(item.toString());
    item = item.reduce();
    console.log(item.toString());
  }

  static NormalizeLatex(latex: string): string {
    console.log(latex);
    let s = latex
      .replace(/\\left/g, "")
      .replace(/\\right/g, "")
      .replace(/\\l/g, "L")
      .replace(" ", "");
    console.log(s);

    return s;
  }
}
