use anyhow::{anyhow, Context};
use dotenvy::dotenv;
use reqwest::blocking::Client;
use std::{
    collections::{HashMap, VecDeque},
    env,
    time::Duration,
};
use titles::KEVIN_BACON;
use wiki_response::WikiResponse;

pub struct WikipediaCrawler {
    client: Client,
}

impl WikipediaCrawler {
    /// Create a new instance of an object to search pages
    ///
    /// # Errors
    ///
    /// `new` errors if:
    ///   - The environment variable `CONTACT` cannot be found (used to create
    ///     user agent for http requests)
    ///   - There is an error encountered while creating the http client
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let contact = env::var("CONTACT")?;
        let user_agent = format!("MyWikiCrawler ({contact})");

        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(5))
            .build()
            .context("Error creating http client")?;

        Ok(Self { client })
    }

    /// Execute the main crawl process.
    ///
    /// # Errors
    ///
    /// This function errors if it fails to find a successful path after
    /// exhausting all found links.
    pub fn crawl(&self, start_title: &str) -> anyhow::Result<Vec<String>> {
        if start_title == KEVIN_BACON {
            return Ok(vec![KEVIN_BACON.to_string()]);
        }

        let mut queue = VecDeque::from([start_title.to_string()]);
        let mut parents = HashMap::new();

        'search: while let Some(cur_title) = queue.pop_front() {
            println!("Crawling {cur_title}");

            let linked_titles = match self.get_linked_titles(&cur_title) {
                Ok(linked_titles) => linked_titles,
                Err(e) => {
                    println!("Failed to get linked titles for page '{cur_title}': {e}");
                    continue;
                }
            };

            println!(
                "Got linked titles for page '{cur_title}'; length: {}",
                linked_titles.len()
            );

            for linked_title in linked_titles {
                if parents.contains_key(&linked_title) {
                    continue;
                }

                parents.insert(linked_title.to_string(), cur_title.to_string());

                if linked_title == KEVIN_BACON {
                    println!("Found target");
                    break 'search;
                }

                queue.push_back(linked_title.to_string());
            }
        }

        println!("Crawl finished.");

        let path = Self::get_path(start_title, &parents);
        println!("{:?}", path.as_ref());

        path.map_err(|_| anyhow::Error::msg("Could not find path to Kevin Bacon"))
    }

    /// Collect all titles linked to in the article with the given title.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///   - Any http request fails
    ///   - Any http response status is not within 200-299
    ///   - Returned JSON is invalid or otherwise not decodable
    pub fn get_linked_titles(&self, title: &str) -> anyhow::Result<Vec<String>> {
        let url = "https://en.wikipedia.org/w/api.php";
        let mut params = HashMap::from([
            ("action".to_string(), "query".to_string()),
            ("titles".to_string(), title.to_string()),
            ("prop".to_string(), "links".to_string()),
            ("pllimit".to_string(), "max".to_string()),
            ("format".to_string(), "json".to_string()),
        ]);

        let mut linked_titles = Vec::new();

        loop {
            let resp = self
                .client
                .get(url)
                .query(&params)
                .send()
                .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

            if !resp.status().is_success() {
                return Err(anyhow!("HTTP error {} for page '{}'", resp.status(), title));
            }

            let body_text = resp
                .text()
                .map_err(|e| anyhow!("Failed to read response body for '{}': {}", title, e))?;

            let wiki_resp: WikiResponse = serde_json::from_str(&body_text)
                .map_err(|e| anyhow!("Failed to decode JSON for page '{}': {}", title, e))?;

            for page in wiki_resp.query.pages.values() {
                if let Some(links) = &page.links {
                    linked_titles.extend(
                        links
                            .iter()
                            .filter(|link| link.ns == 0)
                            .map(|link| link.title.clone()),
                    );
                }
            }

            // Handle continuation
            if let Some(cont) = wiki_resp.continuation {
                params.extend(cont);
            } else {
                break;
            }
        }

        Ok(linked_titles)
    }

    fn get_path(
        start_title: &str,
        parents: &HashMap<String, String>,
    ) -> anyhow::Result<Vec<String>> {
        let mut path = vec![KEVIN_BACON.to_string()];

        while let Some(last) = path.last() {
            if last == start_title {
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
