from collections import deque, defaultdict
import functools
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
        self.to_visit: deque[str] = deque([starting_page_title])
        self.parents: defaultdict[str, set[str]] = defaultdict(set)

    def crawl(self) -> list[str] | None:
        if self.start == KEVIN_BACON_TITLE:
            return [KEVIN_BACON_TITLE]

        pages = 0
        while len(self.to_visit) != 0:
            cur_title = self.to_visit.popleft()
            linked_titles = self._get_linked_titles(cur_title)
            pages += 1
            print(f"done {pages} pages")

            for linked_title in linked_titles:
                if linked_title in self.parents:
                    continue

                self.parents[linked_title].add(cur_title)

                if linked_title == KEVIN_BACON_TITLE:
                    return self._get_path()

                self.to_visit.append(linked_title)

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

    # OPTIMIZATION: add a `maxsize` to the cache
    @functools.lru_cache
    def _get_path(
        self,
        starting_path: tuple[str] | None = None,
    ) -> list[str] | None:
        if starting_path is None:
            starting_path = (KEVIN_BACON_TITLE,)

        cur_title = starting_path[0]

        if cur_title == self.start:
            return list(starting_path)

        parents = self.parents[cur_title]

        if len(parents) == 0:
            return None

        resulting_paths: list[list[str]] = []

        for parent in parents:
            new_path = self._get_path((parent,) + starting_path)
            if new_path is not None:
                resulting_paths.append(new_path)

        if len(resulting_paths) == 0:
            return None

        resulting_paths.sort(key=len)
        return resulting_paths[0]
