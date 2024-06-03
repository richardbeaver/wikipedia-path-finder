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
                    return self._get_path()

                self.queue.append(linked_title)
                self.seen.add(linked_title)
                visited_pages += 1

        return None

    def _get_linked_titles(self, title: str) -> set[str]:
        html = requests.get(f"{GET_HTML_URL}/{title}", timeout=5).text
        return self._linked_titles_in_html(html)

    def _linked_titles_in_html(self, html: str) -> set[str]:
        all_links = BeautifulSoup(html, "html.parser").find_all("a")

        linked_titles: set[str] = set()
        for link in all_links:
            href: str | None = link.get("href")

            if href is not None and href.startswith(ARTICLE_LINK_PREFIX):
                linked_title = href[len(ARTICLE_LINK_PREFIX) :]
                linked_titles.add(linked_title)

        return linked_titles

    def _get_path(
        self,
    ) -> list[str]:
        path = [KEVIN_BACON_TITLE]

        while path[-1] != self.start:
            parent = self.parents[path[-1]]
            path.append(parent)

        path.reverse()
        return path
