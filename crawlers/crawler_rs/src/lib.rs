use anyhow::Context;
use scraper::Selector;
use std::collections::{HashMap, HashSet, VecDeque};

const GET_HTML_URL: &str = "https://en.wikipedia.org/api/rest_v1/page/html";
pub const KEVIN_BACON_TITLE: &str = "Kevin_Bacon";
// The returned html links to other articles by relative paths to their title
const ARTICLE_LINK_PREFIX: &str = "./";

pub struct WikipediaCrawler {
    start: String,
}

impl WikipediaCrawler {
    #[must_use]
    pub fn new(starting_page_title: &str) -> Self {
        Self {
            start: starting_page_title.to_string(),
        }
    }

    /// Execute the main crawl process.
    ///
    /// # Errors
    ///
    /// This function errors if it fails to find a successful path after
    /// exhausting all found links.
    pub async fn crawl(&mut self) -> anyhow::Result<Vec<String>> {
        if self.start == KEVIN_BACON_TITLE {
            return Ok(vec![KEVIN_BACON_TITLE.to_string()]);
        }

        let mut seen = HashSet::new();
        let mut parents = HashMap::new();

        // OPTIMIZATION: start with a capacity?
        let mut queue = VecDeque::new();
        queue.push_back(self.start.to_string());

        let mut visited_pages = 0;

        while let Some(cur_title) = queue.pop_front() {
            println!("visited {visited_pages} pages");

            let Ok(linked_titles) = self.get_linked_titles(&cur_title).await else {
                continue;
            };

            for linked_title in linked_titles {
                if seen.contains(&linked_title) {
                    continue;
                }

                parents.insert(linked_title.to_string(), cur_title.to_string());

                if linked_title == KEVIN_BACON_TITLE {
                    let path = self.get_path(&parents)?;
                    return Ok(path);
                }

                queue.push_back(linked_title.to_string());
                seen.insert(linked_title);
                visited_pages += 1;
            }
        }

        Err(anyhow::Error::msg("Could not find path to Kevin Bacon"))
    }

    #[must_use]
    pub fn linked_titles_in_html(html: &str) -> HashSet<String> {
        let parsed = scraper::Html::parse_document(html);

        let Ok(anchor_tags) = Selector::parse("a") else {
            unreachable!("'a' is a valid HTML selector")
        };

        parsed
            .select(&anchor_tags)
            .filter_map(|anchor_tag| {
                anchor_tag
                    .attr("href")
                    .and_then(|link| link.strip_prefix(ARTICLE_LINK_PREFIX))
            })
            .map(String::from)
            .collect()
    }

    async fn get_linked_titles(&self, title: &str) -> reqwest::Result<HashSet<String>> {
        let url = format!("{GET_HTML_URL}/{title}");
        let html = reqwest::get(url).await?.text().await?;
        Ok(Self::linked_titles_in_html(&html))
    }

    fn get_path(&self, parents: &HashMap<String, String>) -> anyhow::Result<Vec<String>> {
        let mut path = Vec::with_capacity(1);
        path.push(KEVIN_BACON_TITLE.to_string());

        while let Some(last) = path.last() {
            if last == &self.start {
                break;
            }

            let parent = parents
                .get(last)
                .context(format!("Parent of {last} should be present in parents map"))?;
            path.push(parent.to_string());
        }

        path.reverse();
        Ok(path)
    }
}
