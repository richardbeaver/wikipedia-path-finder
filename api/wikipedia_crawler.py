from collections import deque
import requests
from bs4 import BeautifulSoup


GET_HTML_URL = "https://en.wikipedia.org/api/rest_v1/page/html"
KEVIN_BACON_TITLE = "Kevin_Bacon"
# The returned html links to other articles by relative paths to their title
ARTICLE_LINK_PREFIX = "./"


class WikipediaCrawler:

    # Fields of crawler only contain Wikipedia article titles

    def __init__(self, starting_page_title: str):
        self.start: str = starting_page_title
        self.seen: set[str] = set([starting_page_title])
        self.queue: deque[str] = deque([starting_page_title])
        self.parents: dict[str, str] = {}

    def crawl(self) -> list[str] | None:
        if self.start == KEVIN_BACON_TITLE:
            return [KEVIN_BACON_TITLE]

        visited_pages = 0

        while len(self.queue) != 0:
            print(f"visited {visited_pages} pages")

            cur_title = self.queue.popleft()
            linked_titles = self._get_linked_titles(cur_title)

            for linked_title in linked_titles:
                if linked_title in self.seen:
                    continue

                self.parents[linked_title] = cur_title

                if linked_title == KEVIN_BACON_TITLE:
                    full_path = self._get_path()
                    if full_path is None:
                        return full_path
                    return list(full_path)

                self.queue.append(linked_title)
                self.seen.add(linked_title)
                visited_pages += 1

        return None

    def _get_linked_titles(self, title: str) -> list[str]:
        html = requests.get(f"{GET_HTML_URL}/{title}", timeout=5).text
        return list(self._linked_titles_in_html(html))

    def _linked_titles_in_html(self, html: str) -> set[str]:
        all_links = BeautifulSoup(html, "html.parser").find_all("a")

        linked_titles: set[str] = set()
        for link in all_links:
            href: str | None = link.get("href")

            if href is not None and href.startswith(ARTICLE_LINK_PREFIX):
                linked_title = href[len(ARTICLE_LINK_PREFIX) :]
                linked_titles.add(linked_title)

        return linked_titles

    # Single parent version
    def _get_path(
        self,
    ) -> deque[str] | None:
        path = deque([KEVIN_BACON_TITLE])

        while path[0] != self.start:
            parent = self.parents[path[0]]
            path.appendleft(parent)

        return path

    # "set of parents" version
    # def _get_path_rec(
    #     self,
    #     # next_parent: str,
    #     starting_path: deque[str] | None = None,
    # ) -> deque[str] | None:
    #     # if len(starting_path) == 0:
    #     #     if next_parent is not KEVIN_BACON_TITLE:
    #     #         raise ValueError(
    #     #             "First title must be kevin bacon title if starting path is empty"
    #     #         )

    #     assert starting_path is None or starting_path[-1] == KEVIN_BACON_TITLE

    #     if starting_path is None:
    #         starting_path = deque([KEVIN_BACON_TITLE])

    #     cur_title = starting_path[0]

    #     # if next_parent == self.start:
    #     #     starting_path.appendleft(next_parent)
    #     #     return starting_path

    #     parents = self.parents[cur_title]
    #     if len(parents) == 0:
    #         return None

    #     # shortest_path: deque[str] | None = None
    #     shortest_path_parent: str | None = None

    #     # starting_path.appendleft(next_parent)
    #     for parent in parents:
    #         starting_path.appendleft(parent)
    #         new_path = self._get_path(starting_path)
    #         starting_path.popleft(parent)

    #         if new_path is None:
    #             continue

    #         # if shortest_path is None or len(new_path) < len(shortest_path):
    #         #     shortest_path = new_path

    #     if shortest_path is None:
    #         return None

    #     return shortest_path


if __name__ == "__main__":
    FOOTLOOSE_TITLE = "Footloose_(1984_film)"
    HERBERT_ROSS_TITLE = "Herbert_Ross"

    crawler = WikipediaCrawler(HERBERT_ROSS_TITLE)
    assert crawler.crawl() == [HERBERT_ROSS_TITLE, FOOTLOOSE_TITLE, KEVIN_BACON_TITLE]
