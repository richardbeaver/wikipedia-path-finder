import pytest
from crawler_py.crawler import WikipediaCrawler, KEVIN_BACON_TITLE

FOOTLOOSE_TITLE = "Footloose_(1984_film)"
HERBERT_ROSS_TITLE = "Herbert_Ross"
FRIDAY_THE_13TH_TITLE = "Friday_the_13th_(1980_film)"
CITY_ON_A_HILL = "City_on_a_Hill_(TV_series)"
AMANDA_CLAYTON_TITLE = "Amanda_Clayton"
THE_BET_TITLE = "The_Bet_(2016_film)"


def test_starting_at_kevin_bacon():
    crawler = WikipediaCrawler(KEVIN_BACON_TITLE)
    assert crawler.crawl() == [KEVIN_BACON_TITLE]


def test_one_hop_1():
    crawler = WikipediaCrawler(FOOTLOOSE_TITLE)
    assert crawler.crawl() == [FOOTLOOSE_TITLE, KEVIN_BACON_TITLE]


def test_one_hop_2():
    crawler = WikipediaCrawler(FRIDAY_THE_13TH_TITLE)
    assert crawler.crawl() == [FRIDAY_THE_13TH_TITLE, KEVIN_BACON_TITLE]


def test_one_hop_3():
    crawler = WikipediaCrawler(CITY_ON_A_HILL)
    assert crawler.crawl() == [CITY_ON_A_HILL, KEVIN_BACON_TITLE]


@pytest.mark.skip
def test_two_hops_1():
    # Runs in about 10 seconds
    # Multiple paths with two hops
    crawler = WikipediaCrawler(HERBERT_ROSS_TITLE)
    result = crawler.crawl()
    assert result is not None
    assert len(result) == 3
    assert result[0] == HERBERT_ROSS_TITLE
    assert result[2] == KEVIN_BACON_TITLE


@pytest.mark.skip
def test_two_hops_2():
    # Runs in about 10 seconds
    crawler = WikipediaCrawler(AMANDA_CLAYTON_TITLE)
    assert crawler.crawl() == [AMANDA_CLAYTON_TITLE, CITY_ON_A_HILL, KEVIN_BACON_TITLE]


@pytest.mark.skip
def test_three_hops():
    # Runs in about 10 minutes
    crawler = WikipediaCrawler(THE_BET_TITLE)
    result = crawler.crawl()
    assert result is not None
    assert len(result) == 4
    assert result[0] == THE_BET_TITLE
    assert result[3] == KEVIN_BACON_TITLE


def test_get_links_in_sample_html():
    html = """
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
</p>
"""
    assert WikipediaCrawler.linked_titles_in_html(html) == set(
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
    )
