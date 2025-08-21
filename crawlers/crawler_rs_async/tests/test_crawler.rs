use crawler_rs_async::WikipediaCrawler;
use std::sync::LazyLock;
use titles::{
    AMANDA_CLAYTON, CITY_ON_A_HILL, FOOTLOOSE, FRIDAY_THE_13TH, GINETTA_GT5_CHALLENGE,
    GRAN_TURISMO_5, GRAN_TURISMO_5_PROLOGUE, GT5, HERBERT_ROSS, KEVIN_BACON, THE_BET,
};

static CRAWLER: LazyLock<WikipediaCrawler> = LazyLock::new(|| WikipediaCrawler::new().unwrap());

#[tokio::test]
async fn starting_at_kevin_bacon() {
    assert_eq!(CRAWLER.crawl(KEVIN_BACON).await.unwrap(), vec![KEVIN_BACON]);
}

#[tokio::test]
async fn one_hop_1() {
    assert_eq!(
        CRAWLER.crawl(FOOTLOOSE).await.unwrap(),
        vec![FOOTLOOSE, KEVIN_BACON]
    );
}

#[tokio::test]
async fn one_hop_2() {
    assert_eq!(
        CRAWLER.crawl(FRIDAY_THE_13TH).await.unwrap(),
        vec![FRIDAY_THE_13TH, KEVIN_BACON]
    );
}

#[tokio::test]
async fn one_hop_3() {
    assert_eq!(
        CRAWLER.crawl(CITY_ON_A_HILL).await.unwrap(),
        vec![CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[tokio::test]
async fn two_hops_1() {
    // There are at least a couple paths of length 4.
    // Two found while testing:
    // - ["Amanda Clayton", "If Loving You Is Wrong", "The Rosie Show", "Kevin Bacon"]
    // - ["Amanda Clayton", "Katey Sagal", "Kyra Sedgwick", "Kevin Bacon"]
    //
    // Must correctly handle ordering of new titles added to the queue to
    // ensure we find the shortest path, which is length 3.
    assert_eq!(
        CRAWLER.crawl(AMANDA_CLAYTON).await.unwrap(),
        vec![AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[ignore = "long execution time"]
#[tokio::test]
async fn two_hops_2() {
    // Runs in about 10 seconds
    // Triggers Action API's chunked responses with `continue` field
    //   - Faulty handling of this field results in a failure
    let result = CRAWLER.crawl(HERBERT_ROSS).await.unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap(), HERBERT_ROSS);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[ignore = "very long execution time"]
#[tokio::test]
async fn three_hops() {
    // Runs in about 1-7 minutes
    let result = CRAWLER.crawl(THE_BET).await.unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result.first().unwrap(), THE_BET);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[tokio::test]
async fn get_linked_titles() {
    assert_eq!(
        CRAWLER.get_linked_titles(GT5).await.unwrap(),
        [
            GINETTA_GT5_CHALLENGE,
            GRAN_TURISMO_5,
            GRAN_TURISMO_5_PROLOGUE
        ]
    );
}
