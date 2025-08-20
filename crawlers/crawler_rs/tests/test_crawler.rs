use std::sync::LazyLock;

use crawler_rs::WikipediaCrawler;
use titles::{
    AMANDA_CLAYTON, CITY_ON_A_HILL, FOOTLOOSE, FRIDAY_THE_13TH, GINETTA_GT5_CHALLENGE,
    GRAN_TURISMO_5, GRAN_TURISMO_5_PROLOGUE, GT5, HERBERT_ROSS, KEVIN_BACON, THE_BET,
};

static CRAWLER: LazyLock<WikipediaCrawler> = LazyLock::new(|| WikipediaCrawler::new().unwrap());

#[test]
fn starting_at_kevin_bacon() {
    assert_eq!(CRAWLER.crawl(KEVIN_BACON).unwrap(), vec![KEVIN_BACON]);
}

#[test]
fn one_hop_1() {
    assert_eq!(
        CRAWLER.crawl(FOOTLOOSE).unwrap(),
        vec![FOOTLOOSE, KEVIN_BACON]
    );
}

#[test]
fn one_hop_2() {
    assert_eq!(
        CRAWLER.crawl(FRIDAY_THE_13TH).unwrap(),
        vec![FRIDAY_THE_13TH, KEVIN_BACON]
    );
}

#[test]
fn one_hop_3() {
    assert_eq!(
        CRAWLER.crawl(CITY_ON_A_HILL).unwrap(),
        vec![CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[test]
fn two_hops_1() {
    assert_eq!(
        CRAWLER.crawl(AMANDA_CLAYTON).unwrap(),
        vec![AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[ignore = "long execution time"]
#[test]
fn two_hops_2() {
    // Runs in about 10 seconds
    // Triggers Action API's chunked responses with `continue` field
    //   - Faulty handling of this field results in a failure
    let result = CRAWLER.crawl(HERBERT_ROSS).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap(), HERBERT_ROSS);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[ignore = "very long execution time"]
#[test]
fn three_hops() {
    // Runs in about 1-7 minutes
    let result = CRAWLER.crawl(THE_BET).unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result.first().unwrap(), THE_BET);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[test]
fn get_linked_titles() {
    assert_eq!(
        CRAWLER.get_linked_titles(GT5).unwrap(),
        [
            GINETTA_GT5_CHALLENGE,
            GRAN_TURISMO_5,
            GRAN_TURISMO_5_PROLOGUE
        ]
    );
}
