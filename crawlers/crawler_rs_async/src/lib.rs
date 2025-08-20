use anyhow::{anyhow, Context};
use async_channel::{unbounded, Receiver, Sender};
use dotenvy::dotenv;
use reqwest::Client;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};
use titles::KEVIN_BACON;
use tokio::{sync::watch, time::Duration};
use wiki_response::WikiResponse;

const WORKER_COUNT: usize = 5;

#[derive(Clone)]
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
    ///
    /// # Panics
    ///
    /// Panics if locking the mutex guard fails, which says that a previous
    /// mutex holder panicked while holding the mutex
    pub async fn crawl(&self, start_title: &str) -> anyhow::Result<Vec<String>> {
        if start_title == KEVIN_BACON {
            return Ok(vec![KEVIN_BACON.to_string()]);
        }

        let (title_tx, title_rx) = unbounded();
        let (stop_tx, mut stop_rx) = watch::channel(false);

        let parents = Arc::new(Mutex::new(HashMap::new()));

        let mut handles = vec![];

        title_tx
            .send(start_title.to_string())
            .await
            .context("Error sending starting title through channel")?;

        for id in 0..WORKER_COUNT {
            let handle = tokio::spawn(self.clone().worker(
                id,
                title_tx.clone(),
                title_rx.clone(),
                stop_tx.clone(),
                stop_rx.clone(),
                parents.clone(),
            ));

            handles.push(handle);
        }

        drop(title_tx);
        stop_rx.wait_for(|val| *val).await?;

        println!("Crawl finished.");

        let path = {
            let p = parents.lock().unwrap();
            Self::get_path(start_title, &p)
        };
        println!("{:?}", path.as_ref());

        path.map_err(|_| anyhow::Error::msg("Could not find path to Kevin Bacon"))
    }

    async fn worker(
        self,
        id: usize,
        title_tx: Sender<String>,
        title_rx: Receiver<String>,
        stop_tx: watch::Sender<bool>,
        stop_rx: watch::Receiver<bool>,
        parents: Arc<Mutex<HashMap<String, String>>>,
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

            let Ok(linked_titles) = self.get_linked_titles(&cur_title).await else {
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

                if linked_title == KEVIN_BACON {
                    println!("[Worker {id}] Found target");
                    let _ = stop_tx.send(true);
                    return;
                }

                let _ = title_tx.send(linked_title).await;
            }
        }

        println!("[Worker {id}] exiting");
    }

    /// Collect all titles linked to in the article with the given title.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///   - Any http request fails
    ///   - Any http response status is not within 200-299
    ///   - Returned JSON is invalid or otherwise not decodable
    pub async fn get_linked_titles(&self, title: &str) -> anyhow::Result<Vec<String>> {
        let url = "https://en.wikipedia.org/w/api.php";
        let mut params = vec![
            ("action".to_string(), "query".to_string()),
            ("titles".to_string(), title.to_string()),
            ("prop".to_string(), "links".to_string()),
            ("pllimit".to_string(), "max".to_string()),
            ("format".to_string(), "json".to_string()),
        ];

        let mut linked_titles = Vec::new();

        loop {
            let resp = self
                .client
                .get(url)
                .query(&params)
                .send()
                .await
                .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

            if !resp.status().is_success() {
                return Err(anyhow!("HTTP error {} for page '{}'", resp.status(), title));
            }

            let wiki_resp: WikiResponse = resp
                .json()
                .await
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
                params.extend(cont.iter().map(|(k, v)| (k.clone(), v.to_string())));
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
