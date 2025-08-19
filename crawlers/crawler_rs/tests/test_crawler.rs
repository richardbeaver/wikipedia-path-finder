use crawler_rs::WikipediaCrawler;
use titles::{
    AMANDA_CLAYTON, CITY_ON_A_HILL, FOOTLOOSE, FRIDAY_THE_13TH, GINETTA_GT5_CHALLENGE,
    GRAN_TURISMO_5, GRAN_TURISMO_5_PROLOGUE, GT5, HERBERT_ROSS, KEVIN_BACON, THE_BET,
};

#[test]
fn starting_at_kevin_bacon() {
    let mut crawler = WikipediaCrawler::new(KEVIN_BACON).unwrap();
    assert_eq!(crawler.crawl().unwrap(), vec![KEVIN_BACON]);
}

#[test]
fn one_hop_1() {
    let mut crawler = WikipediaCrawler::new(FOOTLOOSE).unwrap();
    assert_eq!(crawler.crawl().unwrap(), vec![FOOTLOOSE, KEVIN_BACON]);
}

#[test]
fn one_hop_2() {
    let mut crawler = WikipediaCrawler::new(FRIDAY_THE_13TH).unwrap();
    assert_eq!(crawler.crawl().unwrap(), vec![FRIDAY_THE_13TH, KEVIN_BACON]);
}

#[test]
fn one_hop_3() {
    let mut crawler = WikipediaCrawler::new(CITY_ON_A_HILL).unwrap();
    assert_eq!(crawler.crawl().unwrap(), vec![CITY_ON_A_HILL, KEVIN_BACON]);
}

#[ignore = "long execution time"]
#[test]
fn two_hops_1() {
    // Runs in about 5-60 seconds
    // Multiple paths with two hops
    let mut crawler = WikipediaCrawler::new(HERBERT_ROSS).unwrap();
    let result = crawler.crawl().unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap(), HERBERT_ROSS);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[ignore = "long execution time"]
#[test]
fn two_hops_2() {
    // Runs in about 5-20 seconds
    let mut crawler = WikipediaCrawler::new(AMANDA_CLAYTON).unwrap();
    assert_eq!(
        crawler.crawl().unwrap(),
        vec![AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[ignore = "very long execution time"]
#[test]
fn three_hops() {
    // Runs in about 1-7 minutes
    let mut crawler = WikipediaCrawler::new(THE_BET).unwrap();
    let result = crawler.crawl().unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result.first().unwrap(), THE_BET);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[test]
fn get_linked_titles() {
    let crawler = WikipediaCrawler::new("").unwrap();
    assert_eq!(
        crawler.get_linked_titles(GT5).unwrap(),
        [
            GINETTA_GT5_CHALLENGE,
            GRAN_TURISMO_5,
            GRAN_TURISMO_5_PROLOGUE
        ]
    );
}
