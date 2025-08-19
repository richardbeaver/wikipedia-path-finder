use crawler_rs::WikipediaCrawler;
use titles::{
    AMANDA_CLAYTON, CITY_ON_A_HILL, FOOTLOOSE, FRIDAY_THE_13TH, HERBERT_ROSS, KEVIN_BACON, THE_BET,
};

#[test]
fn starting_at_kevin_bacon() {
    let mut crawler = WikipediaCrawler::new(KEVIN_BACON);
    assert_eq!(crawler.crawl().unwrap(), vec![KEVIN_BACON]);
}

#[test]
fn one_hop_1() {
    let mut crawler = WikipediaCrawler::new(FOOTLOOSE);
    assert_eq!(crawler.crawl().unwrap(), vec![FOOTLOOSE, KEVIN_BACON]);
}

#[test]
fn one_hop_2() {
    let mut crawler = WikipediaCrawler::new(FRIDAY_THE_13TH);
    assert_eq!(crawler.crawl().unwrap(), vec![FRIDAY_THE_13TH, KEVIN_BACON]);
}

#[test]
fn one_hop_3() {
    let mut crawler = WikipediaCrawler::new(CITY_ON_A_HILL);
    assert_eq!(crawler.crawl().unwrap(), vec![CITY_ON_A_HILL, KEVIN_BACON]);
}

#[ignore = "long execution time"]
#[test]
fn two_hops_1() {
    // Runs in about 5-60 seconds
    // Multiple paths with two hops
    let mut crawler = WikipediaCrawler::new(HERBERT_ROSS);
    let result = crawler.crawl().unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap(), HERBERT_ROSS);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[ignore = "long execution time"]
#[test]
fn two_hops_2() {
    // Runs in about 5-20 seconds
    let mut crawler = WikipediaCrawler::new(AMANDA_CLAYTON);
    assert_eq!(
        crawler.crawl().unwrap(),
        vec![AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]
    );
}

#[ignore = "very long execution time"]
#[test]
fn three_hops() {
    // Runs in about 1-7 minutes
    let mut crawler = WikipediaCrawler::new(THE_BET);
    let result = crawler.crawl().unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result.first().unwrap(), THE_BET);
    assert_eq!(result.last().unwrap(), KEVIN_BACON);
}

#[test]
fn get_links_in_sample_html() {
    let html = r#"
      <p id="mwEA">
        He is known for directing musical and comedies such as
        <i id="mwEQ">
            <a href="./Goodbye,_Mr._Chips_(1969_film)" id="mwEg" rel="mw:WikiLink" title="Goodbye, Mr. Chips (1969 film)">
            Goodbye, Mr. Chips
            </a>
        </i>
        (1969),
        <i id="mwEw">
            <a href="./The_Owl_and_the_Pussycat_(film)" id="mwFA" rel="mw:WikiLink" title="The Owl and the Pussycat (film)">
            The Owl and the Pussycat
            </a>
        </i>
        (1970),
        <i id="mwFQ">
            <a href="./Play_It_Again,_Sam_(film)" id="mwFg" rel="mw:WikiLink" title="Play It Again, Sam (film)">
            Play It Again, Sam
            </a>
        </i>
        (1972),
        <i id="mwFw">
            <a href="./The_Sunshine_Boys_(1975_film)" id="mwGA" rel="mw:WikiLink" title="The Sunshine Boys (1975 film)">
            The Sunshine Boys
            </a>
        </i>
        ,
        <i id="mwGQ">
            <a href="./Funny_Lady" id="mwGg" rel="mw:WikiLink" title="Funny Lady">
            Funny Lady
            </a>
        </i>
        (both 1975),
        <i id="mwGw">
            <a href="./The_Goodbye_Girl" id="mwHA" rel="mw:WikiLink" title="The Goodbye Girl">
            The Goodbye Girl
            </a>
        </i>
        (1977),
        <i id="mwHQ">
            <a href="./California_Suite_(film)" id="mwHg" rel="mw:WikiLink" title="California Suite (film)">
            California Suite
            </a>
        </i>
        (1978), and
        <i id="mwHw">
            <a href="./Pennies_from_Heaven_(1981_film)" id="mwIA" rel="mw:WikiLink" title="Pennies from Heaven (1981 film)">
            Pennies From Heaven
            </a>
        </i>
        (1981). His later films include
        <i id="mwIQ">
            <a href="./Footloose_(1984_film)" id="mwIg" rel="mw:WikiLink" title="Footloose (1984 film)">
            Footloose
            </a>
        </i>
        (1984), and
        <i id="mwIw">
            <a href="./Steel_Magnolias" id="mwJA" rel="mw:WikiLink" title="Steel Magnolias">
            Steel Magnolias
            </a>
        </i>
        (1989). For the drama
        <i id="mwJQ">
            <a href="./The_Turning_Point_(1977_film)" id="mwJg" rel="mw:WikiLink" title="The Turning Point (1977 film)">
            The Turning Point
            </a>
        </i>
        (1977) he received two
        <a class="mw-redirect" href="./Academy_Award" id="mwJw" rel="mw:WikiLink" title="Academy Award">
            Academy Award
        </a>
        .
      </p>"#;

    assert_eq!(
        WikipediaCrawler::linked_titles_in_html(html),
        [
            "Goodbye,_Mr._Chips_(1969_film)",
            "The_Owl_and_the_Pussycat_(film)",
            "Play_It_Again,_Sam_(film)",
            "The_Sunshine_Boys_(1975_film)",
            "Funny_Lady",
            "The_Goodbye_Girl",
            "California_Suite_(film)",
            "Pennies_from_Heaven_(1981_film)",
            "Footloose_(1984_film)",
            "Steel_Magnolias",
            "The_Turning_Point_(1977_film)",
            "Academy_Award",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}
