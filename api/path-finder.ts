import type { Handler, HandlerEvent, HandlerContext } from "@netlify/functions";
import { parse } from "node-html-parser";

const GET_HTML_URL = "https://en.wikipedia.org/api/rest_v1/page/html";
const KEVIN_BACON_TITLE = "Kevin_Bacon";

// Handler `event` object:
// {
//   "path": "Path parameter (original URL encoding)",
//   "httpMethod": "Incoming request’s method name",
//   "headers": {Incoming request headers},
//   "queryStringParameters": {Query string parameters},
//   "body": "A JSON string of the request payload",
//   "isBase64Encoded": "A boolean flag to indicate if the applicable request payload is Base64-encoded"
// }

const handler: Handler = async (
  event: HandlerEvent,
  _context: HandlerContext
) => {
  const startingPageTitle = getPageTitle(event.path);

  const crawler = new WikipediaCrawler(startingPageTitle);
  const path = await crawler.crawl();

  if (path === undefined) {
    return {
      statusCode: 404,
      body: JSON.stringify({ path }),
    };
  }
  return {
    statusCode: 200,
    body: JSON.stringify({ path }),
  };
};

// ==================

function getPageTitle(url: string): string {
  const pathPieces: string[] = url.split("/");
  return pathPieces[pathPieces.length - 1];
}

class Queue {
  queue: string[];
  constructor(...startingItem: string[]) {
    this.queue = startingItem;
  }
  add(newString: string) {
    this.queue.push(newString);
  }
  remove() {
    return this.queue.shift();
  }
  peek() {
    return this.queue[0];
  }
  size() {
    return this.queue.length;
  }
  empty() {
    return this.queue.length === 0;
  }
}

class WikipediaCrawler {
  // Fields of crawler only contain Wikipedia article titles
  start: string;
  visited = new Set<string>();
  toVisit = new Queue();
  parents = new Map<string, string | null>();

  constructor(startingPageTitle: string) {
    this.start = startingPageTitle;
    this.toVisit.add(startingPageTitle);
    this.parents.set(startingPageTitle, null);
  }

  async crawl(): Promise<string[] | undefined> {
    if (this.start === KEVIN_BACON_TITLE) {
      return [KEVIN_BACON_TITLE];
    }

    while (!this.toVisit.empty()) {
      const curTitle = this.toVisit.remove() ?? "";
      if (curTitle === "") {
        continue;
      }
      const linkedTitles: Set<string> = await this.getLinkedTitles(curTitle);

      for (const linkedTitle of linkedTitles) {
        if (linkedTitle === KEVIN_BACON_TITLE) {
          this.parents.set(KEVIN_BACON_TITLE, curTitle);
          return this.getPath();
        }
        if (this.visited.has(linkedTitle)) {
          continue;
        }
        if (!this.parents.has(linkedTitle)) {
          this.toVisit.add(linkedTitle);
          this.parents.set(linkedTitle, curTitle);
        }
      }

      this.visited.add(curTitle);
    }
  }

  async getLinkedTitles(title: string): Promise<Set<string>> {
    const html = await this.getHTMLForTitle(title);

    const root = parse(html);
    const allLinkedTitles = root
      .getElementsByTagName("a")
      .flatMap((element) => element.getAttribute("href") ?? [])
      // Wikipedia's links to other articles are all given by relative paths
      .filter((link) => link.startsWith("./"))
      .map((relPath) => relPath.slice(2));

    return new Set(allLinkedTitles);
  }

  async getHTMLForTitle(title: string): Promise<string> {
    const requestURL = `${GET_HTML_URL}/${title}`;
    const response = await fetch(requestURL);
    return response.text();
  }

  getPath(): string[] {
    const path = [KEVIN_BACON_TITLE];

    let current = KEVIN_BACON_TITLE;
    while (true) {
      const parent = this.parents.get(current);
      if (parent === undefined || parent === null) {
        break;
      }
      path.push(parent);
      current = parent;
    }

    return path.reverse();
  }
}

export { handler };
