from collections import deque
import requests
from bs4 import BeautifulSoup

GET_HTML_URL = "https://en.wikipedia.org/api/rest_v1/page/html"
KEVIN_BACON_TITLE = "Kevin_Bacon"
ARTICLE_LINK_PREFIX = "./"


class WikipediaCrawler:

    # Fields of crawler only contain Wikipedia article titles

    def __init__(self, starting_page_title: str):
        self.start: str = starting_page_title
        self.visited: set[str] = set()
        self.to_visit: deque[str] = deque([starting_page_title])
        # self.parents: dict[str, str | None] = {starting_page_title: None}

    # def crawl(self) -> list[str] | None:
    def get_min_hops(self) -> int | None:
        if self.start == KEVIN_BACON_TITLE:
            return 0

        hops = 0
        pages = 0
        while len(self.to_visit) != 0:
            hops += 1
            print("hops =", hops)

            for _ in range(len(self.to_visit)):
                cur_title = self.to_visit.popleft()
                linked_titles = self._get_linked_titles(cur_title)
                pages += 1
                print(f"done {pages} pages")

                for linked_title in linked_titles:
                    if linked_title == KEVIN_BACON_TITLE:
                        # self.parents[linked_title] = cur_title
                        # return self._get_path()
                        return hops

                    if linked_title in self.visited:
                        continue

                    # if linked_title not in self.parents:
                    self.to_visit.append(linked_title)
                    # self.parents[linked_title] = cur_title

                self.visited.add(cur_title)

        return None

    def _get_linked_titles(self, title: str) -> list[str]:
        html = requests.get(f"{GET_HTML_URL}/{title}", timeout=5).text
        return self._linked_titles_in_html(html)

    def _linked_titles_in_html(self, html: str) -> list[str]:
        all_links = BeautifulSoup(html, "html.parser").find_all("a")

        # linked_titles: set[str] = set()
        linked_titles: list[str] = []
        for link in all_links:
            href: str | None = link.get("href")
            # The returned html links to other articles by relative paths to their title
            if href is not None and href.startswith(ARTICLE_LINK_PREFIX):
                linked_title = href[len(ARTICLE_LINK_PREFIX) :]
                linked_titles.append(linked_title)

        return linked_titles

    # def _get_path(self) -> list[str]:
    #     path: deque[str] = deque([KEVIN_BACON_TITLE])

    #     current = KEVIN_BACON_TITLE
    #     while True:
    #         parent = self.parents.get(current)
    #         if parent is None:
    #             break
    #         path.appendleft(parent)
    #         current = parent

    #     return list(path)
