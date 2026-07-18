export function TODO(msg: string = "todo") {
  throw new Error(msg)
}

export class HTMLParser {
  static parse<T extends HTMLElement>(element: string, query: string): T {
    let d = new DOMParser().parseFromString(element, 'text/html');
    return d.querySelector<T>(query)!;
  }
}
