import pytest
from crawlers.crawler_py.crawler import WikipediaCrawler
from titles.titles import (
    KEVIN_BACON,
    FOOTLOOSE,
    HERBERT_ROSS,
    FRIDAY_THE_13TH,
    CITY_ON_A_HILL,
    AMANDA_CLAYTON,
    THE_BET,
)


def test_starting_at_kevin_bacon():
    crawler = WikipediaCrawler(KEVIN_BACON)
    assert crawler.crawl() == [KEVIN_BACON]


def test_one_hop_1():
    crawler = WikipediaCrawler(FOOTLOOSE)
    assert crawler.crawl() == [FOOTLOOSE, KEVIN_BACON]


def test_one_hop_2():
    crawler = WikipediaCrawler(FRIDAY_THE_13TH)
    assert crawler.crawl() == [FRIDAY_THE_13TH, KEVIN_BACON]


def test_one_hop_3():
    crawler = WikipediaCrawler(CITY_ON_A_HILL)
    assert crawler.crawl() == [CITY_ON_A_HILL, KEVIN_BACON]


@pytest.mark.skip
def test_two_hops_1():
    # Runs in about 10 seconds
    # Multiple paths with two hops
    crawler = WikipediaCrawler(HERBERT_ROSS)
    result = crawler.crawl()
    assert result is not None
    assert len(result) == 3
    assert result[0] == HERBERT_ROSS
    assert result[2] == KEVIN_BACON


@pytest.mark.skip
def test_two_hops_2():
    # Runs in about 10 seconds
    crawler = WikipediaCrawler(AMANDA_CLAYTON)
    assert crawler.crawl() == [AMANDA_CLAYTON, CITY_ON_A_HILL, KEVIN_BACON]


@pytest.mark.skip
def test_three_hops():
    # Runs in about 10 minutes
    crawler = WikipediaCrawler(THE_BET)
    result = crawler.crawl()
    assert result is not None
    assert len(result) == 4
    assert result[0] == THE_BET
    assert result[3] == KEVIN_BACON


def test_get_linked_titles():
    assert WikipediaCrawler("").get_linked_titles("GT5") == [
        "Ginetta GT5 Challenge",
        "Gran Turismo 5",
        "Gran Turismo 5 Prologue",
    ]
