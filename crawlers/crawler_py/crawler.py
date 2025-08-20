import os
from collections import deque
from dotenv import load_dotenv
import requests
from titles.titles import KEVIN_BACON


class WikipediaCrawler:
    def __init__(self):
        load_dotenv()
        contact = os.getenv("CONTACT")
        user_agent = f"MyWikiCrawler ({contact})"

        self.session = requests.Session()
        self.session.headers.update({"User-Agent": user_agent})

    def crawl(self, start_title: str) -> list[str] | None:
        if start_title == KEVIN_BACON:
            return [KEVIN_BACON]

        queue = deque([start_title])
        parents: dict[str, str] = {}

        visited_pages = 0

        while len(queue) != 0:
            print(f"visited {visited_pages} pages")

            cur_title = queue.popleft()
            linked_titles = self.get_linked_titles(cur_title)

            for linked_title in linked_titles:
                if linked_title in parents:
                    continue

                parents[linked_title] = cur_title

                if linked_title == KEVIN_BACON:
                    return self._get_path(start_title, parents)

                queue.append(linked_title)
                visited_pages += 1

        return None

    def get_linked_titles(self, title: str) -> list[str]:
        url = "https://en.wikipedia.org/w/api.php"
        params = {
            "action": "query",
            "titles": title,
            "prop": "links",
            "pllimit": "max",
            "format": "json",
        }

        linked_titles: list[str] = []
        while True:
            response = self.session.get(url, params=params, timeout=5)

            if response.status_code != 200:
                raise RuntimeError(
                    f"HTTP error {response.status_code} for page '{title}'"
                )

            try:
                response = response.json()
            except ValueError as e:
                raise RuntimeError(
                    f"Failed to decode JSON for page '{title}': {e}"
                ) from e

            pages = response["query"]["pages"]

            for _page_id, page_data in pages.items():
                links = page_data.get("links", [])
                linked_titles.extend(link["title"] for link in links if link["ns"] == 0)

            # Handle continuation
            if "continue" in response:
                params.update(response["continue"])
            else:
                break

        return linked_titles

    @staticmethod
    def _get_path(start_title: str, parents: dict[str, str]) -> list[str]:
        path = [KEVIN_BACON]

        while path[-1] != start_title:
            parent = parents[path[-1]]
            path.append(parent)

        path.reverse()
        return path
