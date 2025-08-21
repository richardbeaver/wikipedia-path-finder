use anyhow::{anyhow, Context};
use dotenvy::dotenv;
use reqwest::Client;
use std::{
    collections::{HashMap, VecDeque},
    env,
    sync::{Arc, Mutex},
};
use titles::KEVIN_BACON;
use tokio::{
    sync::{watch, Barrier},
    time::Duration,
};
use wiki_response::WikiResponse;

#[derive(Clone)]
pub struct WikipediaCrawler {
    client: Client,
    worker_count: u8,
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
    pub fn new(worker_count: u8) -> anyhow::Result<Self> {
        dotenv().ok();
        let contact = env::var("CONTACT")?;
        let user_agent = format!("MyWikiCrawler ({contact})");

        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(5))
            .build()
            .context("Error creating http client")?;

        Ok(Self {
            client,
            worker_count,
        })
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

        let frontier = Arc::new(Mutex::new(VecDeque::from([start_title.to_string()])));
        let next_frontier = Arc::new(Mutex::new(VecDeque::new()));

        let (stop_tx, stop_rx) = watch::channel(false);
        let barrier = Arc::new(Barrier::new(self.worker_count as usize + 1));

        let parents = Arc::new(Mutex::new(HashMap::new()));

        for id in 0..self.worker_count as usize {
            tokio::spawn(self.clone().worker(
                id,
                frontier.clone(),
                next_frontier.clone(),
                stop_tx.clone(),
                stop_rx.clone(),
                parents.clone(),
                barrier.clone(),
            ));
        }

        // Wait for coordinator to exit
        let Ok(Ok(())) =
            tokio::spawn(
                self.clone()
                    .coordinator(frontier, next_frontier, stop_rx, barrier),
            )
            .await
        else {
            println!("Coordinator failed");
            return Err(anyhow::Error::msg(""));
        };

        println!("Crawl finished.");

        let path = {
            let p = parents.lock().unwrap();
            Self::get_path(start_title, &p)
        };
        println!("{:?}", path.as_ref());

        path.map_err(|_| anyhow::Error::msg("Could not find path to Kevin Bacon"))
    }

    async fn coordinator(
        self,
        frontier: Arc<Mutex<VecDeque<String>>>,
        next_frontier: Arc<Mutex<VecDeque<String>>>,
        stop_rx: watch::Receiver<bool>,
        barrier: Arc<Barrier>,
    ) -> anyhow::Result<()> {
        let mut round = 0;

        loop {
            barrier.wait().await; // Wait for workers for the round
            round += 1;
            println!("[Coordinator] End of round {round}");

            if *stop_rx.borrow() {
                println!("[Coordinator] Stopping");
                return Ok(());
            }

            {
                let mut nf = next_frontier.lock().unwrap();
                println!("[Coordinator] Titles collected: {}", nf.len());
                let mut f = frontier.lock().unwrap();

                f.extend(nf.drain(..));
            }

            println!("[Coordinator] Starting next round");
            barrier.wait().await; // Start next round
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn worker(
        self,
        id: usize,
        frontier: Arc<Mutex<VecDeque<String>>>,
        next_frontier: Arc<Mutex<VecDeque<String>>>,
        stop_tx: watch::Sender<bool>,
        stop_rx: watch::Receiver<bool>,
        parents: Arc<Mutex<HashMap<String, String>>>,
        barrier: Arc<Barrier>,
    ) {
        loop {
            'this_round: loop {
                if *stop_rx.borrow() {
                    println!("[Worker {id}] stopping");
                    break 'this_round;
                }

                let Some(cur_title) = frontier.lock().unwrap().pop_front() else {
                    break 'this_round;
                };

                let linked_titles = match self.get_linked_titles(&cur_title).await {
                    Ok(linked_titles) => linked_titles,
                    Err(e) => {
                        println!(
                            "[Worker {id}] Failed to get linked titles for page '{cur_title}': {e}"
                        );
                        continue;
                    }
                };

                println!(
                    "[Worker {id}] Got linked titles for page '{cur_title}'; length: {}",
                    linked_titles.len()
                );

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
                        break 'this_round;
                    }

                    next_frontier.lock().unwrap().push_back(linked_title);
                }
            }

            barrier.wait().await; // round finished
            barrier.wait().await; // wait for coordinator to swap queues
        }
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
                .await
                .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

            if !resp.status().is_success() {
                return Err(anyhow!("HTTP error {} for page '{}'", resp.status(), title));
            }

            let body_text = resp
                .text()
                .await
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
