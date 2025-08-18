use anyhow::Context;
use async_channel::{unbounded, Receiver, Sender};
use scraper::Selector;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::watch, time};

const GET_HTML_URL: &str = "https://en.wikipedia.org/api/rest_v1/page/html";
pub const KEVIN_BACON_TITLE: &str = "Kevin_Bacon";
// The returned html links to other articles by relative paths to their title
const ARTICLE_LINK_PREFIX: &str = "./";

const WORKER_COUNT: usize = 5;

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
    ///
    /// # Panics
    ///
    /// Panics if locking the mutex guard fails, which says that a previous
    /// mutex holder panicked while holding the mutex
    pub async fn crawl(&mut self) -> anyhow::Result<Vec<String>> {
        if self.start == KEVIN_BACON_TITLE {
            return Ok(vec![KEVIN_BACON_TITLE.to_string()]);
        }

        let (title_tx, title_rx) = unbounded();
        let (stop_tx, mut stop_rx) = watch::channel(false);

        let parents = Arc::new(Mutex::new(HashMap::new()));
        let client = reqwest::Client::builder()
            .build()
            .context("Error creating http client")?;

        let mut handles = vec![];

        title_tx
            .send(self.start.to_string())
            .await
            .context("Error sending starting title through channel")?;

        for id in 0..WORKER_COUNT {
            let title_tx = title_tx.clone();
            let title_rx = title_rx.clone();
            let stop_tx = stop_tx.clone();
            let stop_rx = stop_rx.clone();
            let parents = parents.clone();
            let client = client.clone();

            let handle = tokio::spawn(Self::worker(
                id, title_tx, title_rx, stop_tx, stop_rx, parents, client,
            ));

            handles.push(handle);
        }

        drop(title_tx);
        stop_rx.wait_for(|val| *val).await?;

        println!("Crawl finished.");

        let path = {
            let p = parents.lock().unwrap();
            self.get_path(&p)
        };
        println!("{:?}", path.as_ref());

        path.map_err(|_| anyhow::Error::msg("Could not find path to Kevin Bacon"))
    }

    async fn worker(
        id: usize,
        title_tx: Sender<String>,
        title_rx: Receiver<String>,
        stop_tx: watch::Sender<bool>,
        stop_rx: watch::Receiver<bool>,
        parents: Arc<Mutex<HashMap<String, String>>>,
        client: reqwest::Client,
    ) {
        loop {
            if *stop_rx.borrow() {
                println!("[Worker {id}] stopping");
                return;
            }

            let Ok(cur_title) = title_rx.recv().await else {
                break;
            };

            println!("[Worker {id}] Crawling {cur_title}");

            let Ok(linked_titles) = Self::get_linked_titles(client.clone(), &cur_title).await
            else {
                println!("Failed to get linked titles");
                continue;
            };

            println!("[Worker {id}] got linked titles");
            println!("length: {}", linked_titles.len());

            for linked_title in linked_titles {
                {
                    let mut p = parents.lock().unwrap();
                    if p.contains_key(&linked_title) {
                        continue;
                    }
                    p.insert(linked_title.to_string(), cur_title.to_string());
                }

                if linked_title == KEVIN_BACON_TITLE {
                    println!("[Worker {id}] Found target");
                    let _ = stop_tx.send(true);
                    return;
                }

                let _ = title_tx.send(linked_title).await;
            }
        }

        println!("[Worker {id}] exiting");
    }

    #[must_use]
    pub fn linked_titles_in_html(html: &str) -> Vec<String> {
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

    async fn get_linked_titles(
        client: reqwest::Client,
        title: &str,
    ) -> anyhow::Result<Vec<String>> {
        let url = format!("{GET_HTML_URL}/{title}");

        let html = time::timeout(Duration::from_secs(5), client.get(url).send())
            .await??
            .text()
            .await?;

        Ok(Self::linked_titles_in_html(&html))
    }

    fn get_path(&self, parents: &HashMap<String, String>) -> anyhow::Result<Vec<String>> {
        let mut path = vec![KEVIN_BACON_TITLE.to_string()];

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
