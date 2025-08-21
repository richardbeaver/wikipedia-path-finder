use crawler_rs_async_channels::WikipediaCrawler;
use std::sync::LazyLock;
use titles::{
    AMANDA_CLAYTON, CITY_ON_A_HILL, CLINT_EASTWOOD, CURTIS_HANSON, FOOTLOOSE, FRIDAY_THE_13TH,
    GINETTA_GT5_CHALLENGE, GRAN_TURISMO_5, GRAN_TURISMO_5_PROLOGUE, GT5, HERBERT_ROSS, KEVIN_BACON,
    THE_BET,
};

static CRAWLER: LazyLock<WikipediaCrawler> = LazyLock::new(|| WikipediaCrawler::new(5).unwrap());

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
    // 1 worker  - 8.5-9s
    // 5 workers - 2-2.5s

    let result = CRAWLER.crawl(HERBERT_ROSS).await.unwrap();
    // This implementation seems to be sometimes ending with Curtis Hanson as
    // the second name in the final path depending on the number of workers
    assert!(
        result == vec![HERBERT_ROSS, CURTIS_HANSON, KEVIN_BACON]
            || result == vec![HERBERT_ROSS, CLINT_EASTWOOD, KEVIN_BACON]
    );
}

#[ignore = "long execution time"]
#[tokio::test]
async fn three_hops() {
    // 1 worker  - 6-6.5s
    // 5 workers - usually 1.5-2.5s, sometimes up to 5s

    assert_eq!(
        CRAWLER.crawl(THE_BET).await.unwrap(),
        vec![THE_BET, AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]
    );
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
