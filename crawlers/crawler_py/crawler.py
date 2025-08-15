from collections import deque
import re
import requests
from bs4 import BeautifulSoup, Tag

GET_HTML_URL = "https://en.wikipedia.org/api/rest_v1/page/html"
KEVIN_BACON_TITLE = "Kevin_Bacon"
# The returned html links to other articles by relative paths to their title
ARTICLE_LINK_PREFIX = "./"


class WikipediaCrawler:

    # Fields of crawler only contain Wikipedia article titles

    def __init__(self, starting_page_title: str):
        self.start: str = starting_page_title

    def crawl(self) -> list[str] | None:
        if self.start == KEVIN_BACON_TITLE:
            return [KEVIN_BACON_TITLE]

        queue = deque([self.start])
        parents: dict[str, str] = {}

        visited_pages = 0

        while len(queue) != 0:
            print(f"visited {visited_pages} pages")

            cur_title = queue.popleft()
            linked_titles = self._get_linked_titles(cur_title)

            for linked_title in linked_titles:
                if linked_title in parents:
                    continue

                parents[linked_title] = cur_title

                if linked_title == KEVIN_BACON_TITLE:
                    return self._get_path(parents)

                queue.append(linked_title)
                visited_pages += 1

        return None

    @staticmethod
    def linked_titles_in_html(html: str) -> list[str]:
        def get_title(anchor_tag: Tag) -> str:
            href: str | list[str] = anchor_tag["href"]
            link = href if isinstance(href, str) else href[0]
            return str(link)[len(ARTICLE_LINK_PREFIX) :]

        wiki_link_anchor_tags = BeautifulSoup(html, "html.parser").find_all(
            # filter for href's that start with the artile link prefix
            href=re.compile(f"^{ARTICLE_LINK_PREFIX}")
        )

        return [
            get_title(anchor_tag)
            for anchor_tag in wiki_link_anchor_tags
            if isinstance(anchor_tag, Tag)
        ]

    def _get_linked_titles(self, title: str) -> list[str]:
        html = requests.get(f"{GET_HTML_URL}/{title}", timeout=5).text
        return self.linked_titles_in_html(html)

    def _get_path(self, parents: dict[str, str]) -> list[str]:
        path = [KEVIN_BACON_TITLE]

        while path[-1] != self.start:
            parent = parents[path[-1]]
            path.append(parent)

        path.reverse()
        return path
