use crawler_rs::{KEVIN_BACON_TITLE, WikipediaCrawler};

const FOOTLOOSE_TITLE: &str = "Footloose_(1984_film)";
const HERBERT_ROSS_TITLE: &str = "Herbert_Ross";
const FRIDAY_THE_13TH_TITLE: &str = "Friday_the_13th_(1980_film)";
const CITY_ON_A_HILL: &str = "City_on_a_Hill_(TV_series)";
const AMANDA_CLAYTON_TITLE: &str = "Amanda_Clayton";
const THE_BET_TITLE: &str = "The_Bet_(2016_film)";

#[tokio::test]
async fn starting_at_kevin_bacon() {
    let mut crawler = WikipediaCrawler::new(KEVIN_BACON_TITLE);
    assert_eq!(crawler.crawl().await.unwrap(), vec![KEVIN_BACON_TITLE]);
}

#[tokio::test]
async fn one_hop_1() {
    let mut crawler = WikipediaCrawler::new(FOOTLOOSE_TITLE);
    assert_eq!(
        crawler.crawl().await.unwrap(),
        vec![FOOTLOOSE_TITLE, KEVIN_BACON_TITLE]
    );
}

#[tokio::test]
async fn one_hop_2() {
    let mut crawler = WikipediaCrawler::new(FRIDAY_THE_13TH_TITLE);
    assert_eq!(
        crawler.crawl().await.unwrap(),
        vec![FRIDAY_THE_13TH_TITLE, KEVIN_BACON_TITLE]
    );
}

#[tokio::test]
async fn one_hop_3() {
    let mut crawler = WikipediaCrawler::new(CITY_ON_A_HILL);
    assert_eq!(
        crawler.crawl().await.unwrap(),
        vec![CITY_ON_A_HILL, KEVIN_BACON_TITLE]
    );
}

#[ignore]
#[tokio::test]
async fn two_hops_1() {
    // Runs in about 5-60 seconds
    // Multiple paths with two hops
    let mut crawler = WikipediaCrawler::new(HERBERT_ROSS_TITLE);
    let result = crawler.crawl().await.unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap(), HERBERT_ROSS_TITLE);
    assert_eq!(result.last().unwrap(), KEVIN_BACON_TITLE);
}

#[ignore]
#[tokio::test]
async fn two_hops_2() {
    // Runs in about 5-20 seconds
    let mut crawler = WikipediaCrawler::new(AMANDA_CLAYTON_TITLE);
    assert_eq!(
        crawler.crawl().await.unwrap(),
        vec![AMANDA_CLAYTON_TITLE, CITY_ON_A_HILL, KEVIN_BACON_TITLE]
    );
}

#[ignore]
#[tokio::test]
async fn three_hops() {
    // Runs in about 1-7 minutes
    let mut crawler = WikipediaCrawler::new(THE_BET_TITLE);
    let result = crawler.crawl().await.unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result.first().unwrap(), THE_BET_TITLE);
    assert_eq!(result.last().unwrap(), KEVIN_BACON_TITLE);
}
