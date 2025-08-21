use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct WikiResponse {
    pub query: Query,
    #[serde(rename = "continue")]
    pub continuation: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub pages: HashMap<String, Page>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub ns: u32,
    pub title: String,
}
