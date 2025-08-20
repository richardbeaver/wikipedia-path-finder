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
    GT5,
    GINETTA_GT5_CHALLENGE,
    GRAN_TURISMO_5,
    GRAN_TURISMO_5_PROLOGUE,
)


@pytest.fixture(name="crawler", scope="module")
def create_crawler():
    """Provide a WikipediaCrawler instance for all tests in this module."""
    return WikipediaCrawler()


def test_starting_at_kevin_bacon(crawler: WikipediaCrawler):
    assert crawler.crawl(KEVIN_BACON) == [KEVIN_BACON]


def test_one_hop_1(crawler: WikipediaCrawler):
    assert crawler.crawl(FOOTLOOSE) == [FOOTLOOSE, KEVIN_BACON]


def test_one_hop_2(crawler: WikipediaCrawler):
    assert crawler.crawl(FRIDAY_THE_13TH) == [FRIDAY_THE_13TH, KEVIN_BACON]


def test_one_hop_3(crawler: WikipediaCrawler):
    assert crawler.crawl(CITY_ON_A_HILL) == [CITY_ON_A_HILL, KEVIN_BACON]


def test_two_hops_1(crawler: WikipediaCrawler):
    assert crawler.crawl(AMANDA_CLAYTON) == [
        AMANDA_CLAYTON,
        CITY_ON_A_HILL,
        KEVIN_BACON,
    ]


@pytest.mark.skip
def test_two_hops_2(crawler: WikipediaCrawler):
    # Runs in about 10 seconds
    # Triggers Action API's chunked responses with `continue` field
    #   - Faulty handling of this field results in a failure
    result = crawler.crawl(HERBERT_ROSS)
    assert result is not None
    assert len(result) == 3
    assert result[0] == HERBERT_ROSS
    assert result[2] == KEVIN_BACON


@pytest.mark.skip
def test_three_hops(crawler: WikipediaCrawler):
    # Runs in about 10 minutes
    result = crawler.crawl(THE_BET)
    assert result is not None
    assert len(result) == 4
    assert result[0] == THE_BET
    assert result[3] == KEVIN_BACON


def test_get_linked_titles(crawler: WikipediaCrawler):
    assert crawler.get_linked_titles(GT5) == [
        GINETTA_GT5_CHALLENGE,
        GRAN_TURISMO_5,
        GRAN_TURISMO_5_PROLOGUE,
    ]
