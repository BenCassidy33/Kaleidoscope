export const LAMBDA_CHAR: string = "λ";

export class Utils {
  // credit: https://easings.net/#easeInOutBack
  static easeInOutBack(x: number): number {
    const c1 = 1.70158;
    const c2 = c1 * 1.525;

    return x < 0.5
      ? (Math.pow(2 * x, 2) * ((c2 + 1) * 2 * x - c2)) / 2
      : (Math.pow(2 * x - 2, 2) * ((c2 + 1) * (x * 2 - 2) + c2) + 2) / 2;
  }
}

export function TODO(msg: string = "todo") {
  throw new Error(msg);
}

export class HTMLParser {
  static parse<T extends HTMLElement>(element: string, query: string): T {
    let d = new DOMParser().parseFromString(element, "text/html");
    return d.querySelector<T>(query)!;
  }
}

export class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  add(rhs: Point) {
    this.x += rhs.x;
    this.y += rhs.y;
  }

  sub(rhs: Point) {
    this.x += rhs.x;
    this.y += rhs.y;
  }
}

export class Vec2 {
  base: Point;
  theta: number;
  m: number;

  constructor(base: Point, angle: number, magnitude: number) {
    this.base = base;
    this.theta = angle;
    this.m = magnitude;
  }

  dot(rhs: Vec2) {
    this.base.x *= rhs.base.x;
    this.base.y *= rhs.base.y;
  }

  getPointee(): Point {
    let x2 = this.base.x + this.m * Math.cos(this.theta);
    let y2 = this.base.y + this.m * Math.sin(this.theta);

    return new Point(x2, y2);
  }
}

export class ViewBox {
  static Get(element: Element): {
    x: number;
    y: number;
    width: number;
    height: number;
  } {
    const [a, b, c, d] = element.getAttribute("viewBox")!.split(" ");
    return {
      x: parseInt(a!),
      y: parseInt(b!),
      width: parseInt(c!),
      height: parseInt(d!),
    };
  }
}
